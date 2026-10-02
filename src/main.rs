use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::cell::{Cell, RefCell};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

mod database;
mod i18n;
mod macros;
mod models;
mod rcon;
mod servers;
mod theme;
mod ui;
mod utils;

use database::{bg_info, bg_punish, bg_search, bg_unpunish};
use i18n::{load_lang, save_lang, t, Lang};
use macros::{parse_hex_color, MacroList};
use models::{BgMessage, MyPool, Player, PlayerInfo, Punishment, SortColumn, SortDir};
use rcon::{send_command, RconConfig};
use servers::ServerList;
use theme::{load_theme_kind, save_theme_kind, Theme, ThemeKind};
use ui::{danger_button, gradient_button, load_icon, setup_fonts, smooth_button};
use ui::{MacroEditor, MacroEditorResult};
use ui::{PanelAction, SettingsPanel, WelcomeResult, WelcomeScreen};
use utils::lerp_color;

// ---------- ПРИЛОЖЕНИЕ ----------

struct AccessPanelApp {
    server_list: ServerList,
    pool: Option<MyPool>,
    rcon_cfg: Option<RconConfig>,
    screen: AppScreen,
    search_query: String,
    players: Vec<Player>,
    selected: Option<Player>,
    info: Option<PlayerInfo>,
    status: String,
    loading: bool,
    punish_reason: String,
    punish_duration: String,
    punish_tab: u8,
    sort_column: SortColumn,
    sort_dir: SortDir,
    rx: Receiver<BgMessage>,
    tx: Sender<BgMessage>,
    settings_panel: SettingsPanel,

    welcome_anim: f32,
    welcome_anim_started: bool,
    show_logout_confirm: bool,

    theme: Theme,
    lang: Lang,

    macros: MacroList,
    macro_editor_open: bool,
    macro_edit_id: Option<String>,
    macro_edit_label: String,
    macro_edit_command: String,
    macro_edit_color: String,

    // Анимации
    settings_anim: f32,
    logout_anim: f32,
    macro_editor_anim: f32,
}

enum AppScreen {
    Welcome(WelcomeScreen),
    Main,
    AddServer(WelcomeScreen),
}

impl AccessPanelApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);

        let theme = Theme::from_kind(load_theme_kind());
        theme.apply(&cc.egui_ctx);

        let lang = load_lang();
        let macros = MacroList::load();

        let (tx, rx) = channel();
        let server_list = ServerList::load();

        let screen = if server_list.is_empty() {
            AppScreen::Welcome(WelcomeScreen::default())
        } else {
            AppScreen::Main
        };

        let (pool, rcon_cfg) = if server_list.is_empty() {
            (None, None)
        } else {
            build_connections(&server_list)
        };

        let welcome_anim = match &screen {
            AppScreen::Welcome(_) => 0.0,
            _ => 1.0,
        };

        Self {
            server_list,
            pool,
            rcon_cfg,
            screen,
            search_query: String::new(),
            players: vec![],
            selected: None,
            info: None,
            status: "Готов к работе".into(),
            loading: false,
            punish_reason: String::new(),
            punish_duration: String::new(),
            punish_tab: 0,
            sort_column: SortColumn::Username,
            sort_dir: SortDir::Asc,
            rx,
            tx,
            settings_panel: SettingsPanel::default(),
            welcome_anim,
            welcome_anim_started: false,
            show_logout_confirm: false,
            theme,
            lang,
            macros,
            macro_editor_open: false,
            macro_edit_id: None,
            macro_edit_label: String::new(),
            macro_edit_command: String::new(),
            macro_edit_color: "#a78bfa".into(),
            settings_anim: 0.0,
            logout_anim: 0.0,
            macro_editor_anim: 0.0,
        }
    }

    fn start_welcome_anim(&mut self, screen: AppScreen) {
        self.screen = screen;
        self.welcome_anim = 0.0;
        self.welcome_anim_started = true;
    }

    fn do_search(&mut self) {
        if self.search_query.trim().is_empty() {
            self.status = "Введите ник игрока".into();
            return;
        }
        let pool = match &self.pool {
            Some(p) => p.clone(),
            None => { self.status = "Нет подключения к базе".into(); return; }
        };
        self.status = format!("Поиск '{}'...", self.search_query);
        self.loading = true;
        let q = self.search_query.clone();
        let tx = self.tx.clone();
        thread::spawn(move || bg_search(pool, q, tx));
    }

    fn apply_sort(&mut self) {
        let col = self.sort_column;
        let dir = self.sort_dir;
        self.players.sort_by(|a, b| {
            let ord = match col {
                SortColumn::Uuid => a.uuid.cmp(&b.uuid),
                SortColumn::Username => a.username.to_lowercase().cmp(&b.username.to_lowercase()),
            };
            match dir { SortDir::Asc => ord, SortDir::Desc => ord.reverse() }
        });
    }

    fn toggle_sort(&mut self, col: SortColumn) {
        if self.sort_column == col {
            self.sort_dir = if self.sort_dir == SortDir::Asc { SortDir::Desc } else { SortDir::Asc };
        } else {
            self.sort_column = col;
            self.sort_dir = SortDir::Asc;
        }
        self.apply_sort();
    }

    fn open_card(&mut self, player: Player) {
        let pool = match &self.pool { Some(p) => p.clone(), None => return };
        self.selected = Some(player.clone());
        self.info = None;
        self.loading = true;
        self.punish_reason.clear();
        self.punish_duration.clear();
        self.punish_tab = 0;
        let tx = self.tx.clone();
        thread::spawn(move || bg_info(pool, player.uuid.clone(), player.username.clone(), tx));
    }

    fn reload_card(&mut self) {
        if let Some(p) = self.selected.clone() {
            let pool = match &self.pool { Some(p) => p.clone(), None => return };
            self.info = None;
            self.loading = true;
            let tx = self.tx.clone();
            thread::spawn(move || bg_info(pool, p.uuid.clone(), p.username.clone(), tx));
        }
    }

    fn close_card(&mut self) {
        self.selected = None;
        self.info = None;
        self.punish_reason.clear();
        self.punish_duration.clear();
    }

    fn issue_punishment(&mut self, ptype: &str) {
        let player = match &self.selected { Some(p) => p.clone(), None => return };
        if self.punish_reason.trim().is_empty() {
            self.status = "Укажите причину".into();
            return;
        }

        if let Some(cfg) = &self.rcon_cfg {
            let cmd = match ptype {
                "BAN" => format!("ban {} {}", player.username, self.punish_reason),
                "MUTE" => format!("mute {} {}", player.username, self.punish_reason),
                "KICK" => format!("kick {} {}", player.username, self.punish_reason),
                "WARNING" => format!("warn {} {}", player.username, self.punish_reason),
                _ => String::new(),
            };
            if let Err(e) = send_command(cfg, &cmd) {
                self.status = format!("RCON: {}", e);
                return;
            }
        }

        let pool = match &self.pool { Some(p) => p.clone(), None => return };
        let reason = self.punish_reason.clone();
        let duration: i64 = self.punish_duration.trim().parse().unwrap_or(0);
        let ptype = ptype.to_string();

        self.status = format!("Выдача {} для {}...", ptype, player.username);
        self.loading = true;

        let tx = self.tx.clone();
        let uuid = player.uuid.clone();
        let username = player.username.clone();
        thread::spawn(move || bg_punish(pool, uuid, username, ptype, reason, duration, tx));
    }

    fn unpublish_punishment(&mut self, id: i64) {
        let pool = match &self.pool { Some(p) => p.clone(), None => return };
        self.status = "Снятие наказания...".into();
        self.loading = true;
        let tx = self.tx.clone();
        thread::spawn(move || bg_unpunish(pool, id, tx));
    }

    fn poll_messages(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                BgMessage::SearchDone(players) => {
                    self.players = players;
                    self.apply_sort();
                    self.status = format!("Найдено: {}", self.players.len());
                    self.loading = false;
                }
                BgMessage::InfoDone(info) => { self.info = Some(info); self.loading = false; }
                BgMessage::Error(e) => { self.status = e; self.loading = false; }
                BgMessage::PunishDone(msg) => {
                    self.status = msg;
                    self.loading = false;
                    self.reload_card();
                }
            }
        }
    }

    fn switch_server(&mut self, id: &str) {
        self.server_list.set_active(id);
        let _ = self.server_list.save();

        let (pool, rcon_cfg) = build_connections(&self.server_list);
        self.pool = pool;
        self.rcon_cfg = rcon_cfg;

        self.players.clear();
        self.selected = None;
        self.info = None;
        self.status = format!("Подключено к: {}",
            self.server_list.active().map(|s| s.name.as_str()).unwrap_or("—"));
    }

    fn switch_theme(&mut self, kind: ThemeKind, ctx: &egui::Context) {
        self.theme = Theme::from_kind(kind);
        self.theme.apply(ctx);
        let _ = save_theme_kind(kind);
    }

    fn switch_lang(&mut self, lang: Lang) {
        self.lang = lang;
        let _ = save_lang(lang);
    }

    fn run_macro(&mut self, id: &str) {
        let button = match self.macros.buttons.iter().find(|b| b.id == id) {
            Some(b) => b.clone(),
            None => return,
        };

        let player_name = self.selected.as_ref().map(|p| p.username.clone());
        let lang = self.lang;

        let cmd = if button.command.contains("{player}") {
            match &player_name {
                Some(name) => button.command.replace("{player}", name),
                None => {
                    self.status = t("macro_no_player", lang).into();
                    return;
                }
            }
        } else {
            button.command.clone()
        };

        if let Some(cfg) = &self.rcon_cfg {
            match send_command(cfg, &cmd) {
                Ok(_) => self.status = format!("✓ {}: {}", t("macro_done", lang), button.label),
                Err(e) => self.status = format!("RCON: {}", e),
            }
        } else {
            self.status = t("macro_no_rcon", lang).into();
        }
    }

    fn open_macro_editor_new(&mut self) {
        self.macro_editor_open = true;
        self.macro_edit_id = None;
        self.macro_edit_label = String::new();
        self.macro_edit_command = String::new();
        self.macro_edit_color = "#a78bfa".into();
    }

    fn open_macro_editor_existing(&mut self, id: &str) {
        if let Some(b) = self.macros.buttons.iter().find(|b| b.id == id) {
            self.macro_editor_open = true;
            self.macro_edit_id = Some(id.to_string());
            self.macro_edit_label = b.label.clone();
            self.macro_edit_command = b.command.clone();
            self.macro_edit_color = b.color.clone();
        }
    }

    fn save_macro_editor(&mut self) {
        let lang = self.lang;
        if self.macro_edit_label.trim().is_empty() || self.macro_edit_command.trim().is_empty() {
            self.status = t("macro_fill", lang).into();
            return;
        }

        if let Some(id) = self.macro_edit_id.clone() {
            self.macros.update(
                &id,
                self.macro_edit_label.clone(),
                self.macro_edit_command.clone(),
                self.macro_edit_color.clone(),
            );
        } else {
            self.macros.add(
                self.macro_edit_label.clone(),
                self.macro_edit_command.clone(),
                self.macro_edit_color.clone(),
            );
        }
        let _ = self.macros.save();
        self.macro_editor_open = false;
        self.status = t("macro_saved", self.lang).into();
    }

    fn delete_macro(&mut self) {
        if let Some(id) = self.macro_edit_id.clone() {
            self.macros.remove(&id);
            let _ = self.macros.save();
            self.macro_editor_open = false;
            self.status = t("macro_deleted", self.lang).into();
        }
    }

    fn logout(&mut self) {
        self.show_logout_confirm = true;
    }

    fn confirm_logout(&mut self) {
        let _ = std::fs::remove_file("servers.toml");

        self.server_list = ServerList::default();
        self.pool = None;
        self.rcon_cfg = None;
        self.players.clear();
        self.selected = None;
        self.info = None;
        self.search_query.clear();
        self.status = "Отключено".into();
        self.show_logout_confirm = false;

        self.start_welcome_anim(AppScreen::Welcome(WelcomeScreen::default()));
    }
}

// ---------- ВСПОМОГАТЕЛЬНЫЕ ----------

fn animate_to(current: &mut f32, target: f32, speed: f32) -> bool {
    let diff = target - *current;
    if diff.abs() < 0.001 {
        *current = target;
        return false;
    }
    *current += diff * speed;
    true
}

fn build_connections(server_list: &ServerList) -> (Option<MyPool>, Option<RconConfig>) {
    let server = match server_list.active() {
        Some(s) => s,
        None => return (None, None),
    };

    let pool = database::create_pool_for(&server.mysql).ok();
    let rcon_cfg = RconConfig::new(
        server.rcon.host.clone(),
        server.rcon.port,
        server.rcon.password.clone(),
    );

    (pool, Some(rcon_cfg))
}

fn info_row(ui: &mut egui::Ui, label: &str, value: &str, color: egui::Color32, theme: &Theme) {
    ui.horizontal(|ui| {
        ui.add_sized([170.0, 24.0], egui::Label::new(
            egui::RichText::new(label).color(theme.text_muted).size(13.0)
                .family(egui::FontFamily::Name("inter".into()))));
        ui.add_space(8.0);
        ui.label(egui::RichText::new(value).color(color).size(15.0).strong()
            .family(egui::FontFamily::Name("inter".into())));
    });
    ui.add_space(6.0);
}

fn draw_hamburger_button(ui: &mut egui::Ui, id: egui::Id, theme: &Theme) -> bool {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(44.0, 44.0), egui::Sense::click());
    let painter = ui.painter();
    let lvl = ui.ctx().animate_bool_with_time(id, response.hovered(), 0.3);

    let bg_col = lerp_color(theme.surface_2, theme.accent_soft, lvl);
    painter.rect_filled(rect, egui::CornerRadius::same(10), bg_col);

    let center = rect.center();
    let line_w = 18.0;
    let line_h = 2.0;
    let gap = 6.0;
    let line_color = lerp_color(theme.text, theme.accent_bright, lvl);

    for i in -1..=1 {
        let y = center.y + i as f32 * gap;
        painter.rect_filled(
            egui::Rect::from_center_size(
                egui::pos2(center.x, y),
                egui::vec2(line_w, line_h),
            ),
            egui::CornerRadius::same(1),
            line_color,
        );
    }

    response.clicked()
}

fn draw_macro_chip(
    ui: &mut egui::Ui,
    m: &crate::macros::MacroButton,
    color: egui::Color32,
    _theme: &Theme,
) -> (bool, bool) {
    let text = &m.label;
    let font = egui::FontId::new(13.0, egui::FontFamily::Name("inter".into()));

    let galley = ui.painter().layout_no_wrap(text.clone(), font.clone(), color);
    let w = galley.size().x + 34.0;
    let h = 38.0;

    let (rect, response) = ui.allocate_exact_size(egui::vec2(w, h), egui::Sense::click());

    let hover_lvl = ui.ctx().animate_bool_with_time(
        egui::Id::new(format!("macro_hover_{}", m.id)),
        response.hovered(),
        0.2,
    );

    let painter = ui.painter();

    let bg = egui::Color32::from_rgba_unmultiplied(
        color.r(),
        color.g(),
        color.b(),
        (35.0 + 35.0 * hover_lvl) as u8,
    );
    painter.rect_filled(rect, egui::CornerRadius::same(10), bg);

    let border_alpha = (130.0 + 125.0 * hover_lvl) as u8;
    painter.rect_stroke(
        rect,
        egui::CornerRadius::same(10),
        egui::Stroke::new(
            1.2_f32,
            egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), border_alpha),
        ),
        egui::StrokeKind::Inside,
    );

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        font,
        color,
    );

    let edit = response.secondary_clicked();

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    (response.clicked(), edit)
}

fn draw_add_macro_chip(ui: &mut egui::Ui, theme: &Theme, label: &str) -> bool {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(140.0, 38.0), egui::Sense::click());

    let hover_lvl = ui.ctx().animate_bool_with_time(
        egui::Id::new("macro_hover_add"),
        response.hovered(),
        0.2,
    );

    let painter = ui.painter();

    let bg = lerp_color(theme.surface_2, theme.accent_soft, hover_lvl);
    painter.rect_filled(rect, egui::CornerRadius::same(10), bg);

    painter.rect_stroke(
        rect,
        egui::CornerRadius::same(10),
        egui::Stroke::new(1.2_f32, theme.accent),
        egui::StrokeKind::Inside,
    );

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::new(13.0, egui::FontFamily::Name("inter".into())),
        theme.accent,
    );

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    response.clicked()
}

// ---------- ГЛАВНЫЙ ЭКРАН ----------

impl AccessPanelApp {
    fn show_macros_panel(&mut self, ctx: &egui::Context) {
        let theme = self.theme.clone();
        let lang = self.lang;

        egui::TopBottomPanel::top("macros_panel")
            .frame(
                egui::Frame::NONE
                    .fill(theme.bg)
                    .inner_margin(egui::Margin::symmetric(36, 8)),
            )
            .show(ctx, |ui| {
                let mut to_run: Option<String> = None;
                let mut to_edit: Option<String> = None;
                let mut to_add = false;

                ui.horizontal_wrapped(|ui| {
                    for m in &self.macros.buttons {
                        let col = parse_hex_color(&m.color);
                        let (clicked, edit) = draw_macro_chip(ui, m, col, &theme);
                        if clicked {
                            to_run = Some(m.id.clone());
                        }
                        if edit {
                            to_edit = Some(m.id.clone());
                        }
                        ui.add_space(8.0);
                    }

                    if draw_add_macro_chip(ui, &theme, t("macro_add", lang)) {
                        to_add = true;
                    }
                });

                if to_add {
                    self.open_macro_editor_new();
                }
                if let Some(id) = to_run {
                    self.run_macro(&id);
                }
                if let Some(id) = to_edit {
                    self.open_macro_editor_existing(&id);
                }
            });
    }

    fn show_main(&mut self, ctx: &egui::Context) {
        let theme = self.theme.clone();
        let lang = self.lang;

        egui::TopBottomPanel::top("header")
            .frame(egui::Frame::NONE.fill(theme.bg).inner_margin(egui::Margin::symmetric(36, 20)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(16.0, 30.0), egui::Sense::hover());
                    ui.painter().circle_filled(rect.center(), 8.0, theme.accent);
                    ui.painter().circle_filled(rect.center(), 14.0,
                        egui::Color32::from_rgba_unmultiplied(
                            theme.accent.r(), theme.accent.g(), theme.accent.b(), 45));
                    ui.add_space(8.0);

                    let active_name = self.server_list.active()
                        .map(|s| s.name.as_str())
                        .unwrap_or("Без сервера");

                    ui.label(egui::RichText::new(t("app_title", lang)).color(theme.text).size(28.0).strong()
                        .family(egui::FontFamily::Name("inter".into())));
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(format!("·  {}", active_name))
                        .color(theme.text_muted).size(14.0)
                        .family(egui::FontFamily::Name("inter".into())));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if draw_hamburger_button(ui, egui::Id::new("btn_settings"), &theme) {
                            self.settings_panel.open = !self.settings_panel.open;
                        }

                        ui.add_space(10.0);

                        let lang_label = lang.label();
                        let next_lang = match lang {
                            Lang::Ru => Lang::En,
                            Lang::En => Lang::Ru,
                        };
                        if smooth_button(ui, lang_label, egui::vec2(60.0, 44.0),
                                         egui::Id::new("btn_lang"),
                                         theme.surface_2, theme.accent_soft, theme.text).clicked() {
                            self.switch_lang(next_lang);
                        }
                    });
                });
            });

        egui::TopBottomPanel::top("search")
            .frame(egui::Frame::NONE.fill(theme.bg).inner_margin(egui::Margin::symmetric(36, 8)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let response = ui.add_sized(
                        [720.0, 52.0],
                        egui::TextEdit::singleline(&mut self.search_query)
                            .hint_text(egui::RichText::new(t("search_hint", lang)).color(theme.hint))
                            .text_color(theme.text)
                            .margin(egui::Margin::symmetric(14, 14))
                            .font(egui::FontId::new(15.0, egui::FontFamily::Name("inter".into()))),
                    );
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.do_search();
                    }
                    ui.add_space(8.0);
                    if gradient_button(ui, t("search", lang), egui::vec2(160.0, 52.0),
                                       egui::Id::new("btn_search"), &theme).clicked() {
                        self.do_search();
                    }
                    ui.add_space(8.0);
                    if smooth_button(ui, t("refresh", lang), egui::vec2(170.0, 52.0),
                                     egui::Id::new("btn_refresh"),
                                     theme.surface_2, theme.accent_soft, theme.text).clicked() {
                        if let Some(p) = self.selected.clone() { self.open_card(p); }
                        else { self.do_search(); }
                    }
                });
                ui.add_space(10.0);
                ui.label(egui::RichText::new(&self.status).color(theme.text_muted).size(12.0)
                    .family(egui::FontFamily::Name("inter".into())));
            });

        self.show_macros_panel(ctx);

        let mut sort_clicked: Option<SortColumn> = None;

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme.bg).inner_margin(egui::Margin::symmetric(36, 20)))
            .show(ctx, |ui| {
                egui::Frame::NONE.fill(theme.surface).corner_radius(18.0).inner_margin(24.0).show(ui, |ui| {
                    ui.set_min_size(ui.available_size());
                    if self.players.is_empty() {
                        ui.add_space(60.0);
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new(t("empty_results", lang))
                                .color(theme.text_muted).size(15.0)
                                .family(egui::FontFamily::Name("inter".into())));
                        });
                    } else {
                        let mut to_open: Option<Player> = None;

                        let uuid_arrow = if self.sort_column == SortColumn::Uuid {
                            if self.sort_dir == SortDir::Asc { " ↑" } else { " ↓" }
                        } else { "" };
                        let name_arrow = if self.sort_column == SortColumn::Username {
                            if self.sort_dir == SortDir::Asc { " ↑" } else { " ↓" }
                        } else { "" };

                        let uuid_label = format!("{}{}", t("uuid", lang), uuid_arrow);
                        let name_label = format!("{}{}", t("nickname", lang), name_arrow);

                        ui.horizontal(|ui| {
                            let resp_uuid = ui.add_sized(
                                [400.0, 28.0],
                                egui::Label::new(egui::RichText::new(&uuid_label).color(theme.text_muted).size(12.0)
                                    .family(egui::FontFamily::Name("inter".into()))).sense(egui::Sense::click())
                            );
                            if resp_uuid.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
                            if resp_uuid.clicked() { sort_clicked = Some(SortColumn::Uuid); }

                            let resp_name = ui.add_sized(
                                [250.0, 28.0],
                                egui::Label::new(egui::RichText::new(&name_label).color(theme.text_muted).size(12.0)
                                    .family(egui::FontFamily::Name("inter".into()))).sense(egui::Sense::click())
                            );
                            if resp_name.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
                            if resp_name.clicked() { sort_clicked = Some(SortColumn::Username); }
                        });

                        ui.add_space(4.0);

                        egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                            TableBuilder::new(ui)
                                .striped(false)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                .column(Column::exact(400.0))
                                .column(Column::exact(250.0))
                                .header(0.0, |_| {})
                                .body(|mut body| {
                                    for p in &self.players {
                                        body.row(48.0, |mut row| {
                                            row.col(|ui| {
                                                ui.label(egui::RichText::new(&p.uuid).color(theme.text).size(14.0)
                                                    .family(egui::FontFamily::Name("inter".into())));
                                            });
                                            row.col(|ui| {
                                                let resp = ui.label(egui::RichText::new(&p.username)
                                                    .color(theme.accent).size(15.0).strong()
                                                    .family(egui::FontFamily::Name("inter".into())));
                                                if resp.clicked() { to_open = Some(p.clone()); }
                                            });
                                        });
                                    }
                                });
                        });

                        if let Some(p) = to_open { self.open_card(p); }
                    }
                });
            });

        if let Some(col) = sort_clicked {
            self.toggle_sort(col);
        }

        if self.selected.is_some() || self.info.is_some() {
            self.show_card(ctx);
        }

        if self.show_logout_confirm || self.logout_anim > 0.01 {
            self.show_logout_dialog(ctx);
        }
    }

    fn show_logout_dialog(&mut self, ctx: &egui::Context) {
        let theme = self.theme.clone();
        let lang = self.lang;
        let screen = ctx.content_rect();

        let target = if self.show_logout_confirm { 1.0 } else { 0.0 };
        let animating = animate_to(&mut self.logout_anim, target, 0.2);
        if animating {
            ctx.request_repaint();
        }

        if self.logout_anim < 0.01 {
            return;
        }

        let anim = self.logout_anim;

        ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("logout_overlay")))
            .rect_filled(screen, egui::CornerRadius::ZERO,
                egui::Color32::from_rgba_unmultiplied(0x0a, 0x08, 0x10, (180.0 * anim) as u8));

        let scale = 0.9 + 0.1 * anim;
        let dialog_w = 480.0 * scale;
        let dialog_h = 280.0 * scale;
        let dialog_rect = egui::Rect::from_center_size(screen.center(), egui::vec2(dialog_w, dialog_h));

        let mut action: Option<bool> = None;

        egui::Area::new(egui::Id::new("logout_dialog"))
            .fixed_pos(dialog_rect.min)
            .order(egui::Order::Tooltip)
            .show(ctx, |ui| {
                egui::Frame::NONE
                    .fill(theme.surface)
                    .corner_radius(20.0)
                    .inner_margin(28.0)
                    .stroke(egui::Stroke::new(1.0_f32, theme.surface_3))
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(dialog_w - 56.0, dialog_h - 56.0));
                        ui.set_max_size(egui::vec2(dialog_w - 56.0, dialog_h - 56.0));
                        ui.set_opacity(anim);

                        ui.label(egui::RichText::new(t("logout_confirm", lang))
                            .color(theme.text).size(20.0).strong()
                            .family(egui::FontFamily::Name("inter".into())));
                        ui.add_space(14.0);

                        ui.label(egui::RichText::new(t("logout_question", lang))
                            .color(theme.text).size(13.0)
                            .family(egui::FontFamily::Name("inter".into())));
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new(t("logout_warning", lang))
                            .color(theme.danger).size(12.0)
                            .family(egui::FontFamily::Name("inter".into())));

                        ui.add_space(24.0);

                        ui.horizontal(|ui| {
                            if danger_button(ui, t("yes_logout", lang), egui::vec2(160.0, 42.0),
                                             egui::Id::new("btn_confirm_logout"), &theme).clicked() {
                                action = Some(true);
                            }
                            ui.add_space(12.0);
                            if smooth_button(ui, t("cancel", lang), egui::vec2(160.0, 42.0),
                                             egui::Id::new("btn_cancel_logout"),
                                             theme.surface_2, theme.accent_soft, theme.text).clicked() {
                                action = Some(false);
                            }
                        });
                    });
            });

        match action {
            Some(true) => self.confirm_logout(),
            Some(false) => { self.show_logout_confirm = false; }
            None => {}
        }
    }

    fn show_card(&mut self, ctx: &egui::Context) {
        let theme = self.theme.clone();
        let lang = self.lang;
        let username = self.selected.as_ref().map(|p| p.username.clone()).unwrap_or_default();
        let info_clone = self.info.clone();

        let screen = ctx.content_rect();
        let overlay_lvl = ctx.animate_bool_with_time(egui::Id::new("card_overlay"), true, 0.2);

        ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("overlay_bg")))
            .rect_filled(screen, egui::CornerRadius::ZERO,
                egui::Color32::from_rgba_unmultiplied(0x0a, 0x08, 0x10, (overlay_lvl * 180.0) as u8));

        let card_lvl = ctx.animate_bool_with_time(egui::Id::new("card_scale"), true, 0.2);
        let scale = 0.95 + 0.05 * card_lvl;

        let card_w = 820.0 * scale;
        let card_h = 740.0 * scale;
        let card_rect = egui::Rect::from_center_size(screen.center(), egui::vec2(card_w, card_h));

        let shadow_lvl = ctx.animate_bool_with_time(egui::Id::new("card_shadow"), true, 0.25);
        for i in 1..=10 {
            let expand = i as f32 * 3.0 * shadow_lvl;
            let alpha = ((40.0 - i as f32 * 3.5) * shadow_lvl).max(0.0) as u8;
            ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new(format!("shadow_{}", i))))
                .rect_filled(card_rect.expand(expand),
                    egui::CornerRadius::same((20 + i as u8 * 2).min(255)),
                    egui::Color32::from_rgba_unmultiplied(0x00, 0x00, 0x00, alpha));
        }

        let close_clicked = Cell::new(false);
        let unpublish_action: Cell<Option<i64>> = Cell::new(None);
        let punish_action: RefCell<Option<String>> = RefCell::new(None);
        let switch_tab: Cell<Option<u8>> = Cell::new(None);

        let mut reason = self.punish_reason.clone();
        let mut duration = self.punish_duration.clone();
        let mut current_tab = self.punish_tab;

        egui::Area::new(egui::Id::new("card_area"))
            .fixed_pos(card_rect.min)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::NONE
                    .fill(theme.surface)
                    .corner_radius(24.0)
                    .inner_margin(28.0)
                    .stroke(egui::Stroke::new(1.0_f32, theme.surface_3))
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(card_w - 56.0, card_h - 56.0));
                        ui.set_max_size(egui::vec2(card_w - 56.0, card_h - 56.0));

                        ui.horizontal(|ui| {
                            let (r, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                            ui.painter().circle_filled(r.center(), 6.0, theme.accent);
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new(t("player_card", lang)).color(theme.text_muted).size(12.0)
                                .family(egui::FontFamily::Name("inter".into())));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if smooth_button(ui, "X", egui::vec2(36.0, 36.0), egui::Id::new("btn_close_top"),
                                                 theme.surface_2, theme.accent_soft, theme.text).clicked() {
                                    close_clicked.set(true);
                                }
                            });
                        });

                        ui.add_space(10.0);
                        ui.label(egui::RichText::new(&username).color(theme.text).size(24.0).strong()
                            .family(egui::FontFamily::Name("inter".into())));
                        ui.add_space(4.0);

                        if let Some(info) = &info_clone {
                            egui::ScrollArea::vertical().max_height(640.0).show(ui, |ui| {
                                ui.add_space(10.0);

                                egui::Frame::NONE.fill(theme.surface_2).corner_radius(14.0).inner_margin(20.0).show(ui, |ui| {
                                    ui.set_min_width(card_w - 96.0);
                                    info_row(ui, t("group", lang), &info.group.to_uppercase(), theme.accent, &theme);
                                    info_row(ui, t("last_login", lang), &info.lastlogin, theme.success, &theme);
                                    info_row(ui, t("registration", lang), &info.regdate, theme.text, &theme);
                                });

                                ui.add_space(14.0);

                                egui::Frame::NONE.fill(theme.surface_2).corner_radius(14.0).inner_margin(20.0).show(ui, |ui| {
                                    ui.set_min_width(card_w - 96.0);
                                    ui.label(egui::RichText::new(t("issue_punishment", lang)).color(theme.text).size(14.0).strong()
                                        .family(egui::FontFamily::Name("inter".into())));
                                    ui.add_space(8.0);

                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(t("reason", lang)).color(theme.text_muted).size(12.0)
                                            .family(egui::FontFamily::Name("inter".into())));
                                        ui.add_sized([500.0, 32.0],
                                            egui::TextEdit::singleline(&mut reason)
                                                .hint_text(t("reason_hint", lang)).text_color(theme.text)
                                                .margin(egui::Margin::symmetric(10, 6)));
                                    });
                                    ui.add_space(6.0);
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(t("duration", lang)).color(theme.text_muted).size(12.0)
                                            .family(egui::FontFamily::Name("inter".into())));
                                        ui.add_sized([100.0, 32.0],
                                            egui::TextEdit::singleline(&mut duration)
                                                .hint_text(t("minutes", lang)).text_color(theme.text)
                                                .margin(egui::Margin::symmetric(10, 6)));
                                        ui.label(egui::RichText::new(t("minutes_forever", lang)).color(theme.text_muted).size(11.0)
                                            .family(egui::FontFamily::Name("inter".into())));
                                    });
                                    ui.add_space(10.0);
                                    ui.horizontal(|ui| {
                                        if danger_button(ui, "BAN", egui::vec2(90.0, 34.0), egui::Id::new("btn_ban"), &theme).clicked() {
                                            punish_action.replace(Some("BAN".into()));
                                        }
                                        if danger_button(ui, "MUTE", egui::vec2(90.0, 34.0), egui::Id::new("btn_mute"), &theme).clicked() {
                                            punish_action.replace(Some("MUTE".into()));
                                        }
                                        if danger_button(ui, "KICK", egui::vec2(90.0, 34.0), egui::Id::new("btn_kick"), &theme).clicked() {
                                            punish_action.replace(Some("KICK".into()));
                                        }
                                        if danger_button(ui, "WARN", egui::vec2(90.0, 34.0), egui::Id::new("btn_warn"), &theme).clicked() {
                                            punish_action.replace(Some("WARNING".into()));
                                        }
                                    });
                                });

                                ui.add_space(14.0);

                                ui.horizontal(|ui| {
                                    let active_label = format!("{} ({})", t("active", lang), info.active_punishments.len());
                                    let hist_label = format!("{} ({})", t("history", lang), info.history_punishments.len());

                                    let btn_active = if current_tab == 0 {
                                        egui::Button::new(egui::RichText::new(&active_label).color(theme.btn_text).strong()
                                            .family(egui::FontFamily::Name("inter".into())))
                                            .fill(theme.accent).corner_radius(10.0).min_size(egui::vec2(160.0, 34.0))
                                    } else {
                                        egui::Button::new(egui::RichText::new(&active_label).color(theme.text)
                                            .family(egui::FontFamily::Name("inter".into())))
                                            .fill(theme.surface_2).corner_radius(10.0).min_size(egui::vec2(160.0, 34.0))
                                    };
                                    if ui.add(btn_active).clicked() { switch_tab.set(Some(0)); current_tab = 0; }

                                    let btn_hist = if current_tab == 1 {
                                        egui::Button::new(egui::RichText::new(&hist_label).color(theme.btn_text).strong()
                                            .family(egui::FontFamily::Name("inter".into())))
                                            .fill(theme.accent).corner_radius(10.0).min_size(egui::vec2(160.0, 34.0))
                                    } else {
                                        egui::Button::new(egui::RichText::new(&hist_label).color(theme.text)
                                            .family(egui::FontFamily::Name("inter".into())))
                                            .fill(theme.surface_2).corner_radius(10.0).min_size(egui::vec2(160.0, 34.0))
                                    };
                                    if ui.add(btn_hist).clicked() { switch_tab.set(Some(1)); current_tab = 1; }
                                });

                                ui.add_space(10.0);

                                let list: &[Punishment] = if current_tab == 0 {
                                    &info.active_punishments
                                } else {
                                    &info.history_punishments
                                };

                                if list.is_empty() {
                                    let text = if current_tab == 0 { t("no_active", lang) } else { t("no_history", lang) };
                                    ui.label(egui::RichText::new(text).color(theme.text_muted).size(13.0)
                                        .family(egui::FontFamily::Name("inter".into())));
                                } else {
                                    for p in list {
                                        egui::Frame::NONE.fill(theme.surface_2).corner_radius(10.0)
                                            .inner_margin(egui::Margin::symmetric(14, 10)).show(ui, |ui| {
                                            ui.set_min_width(card_w - 96.0);
                                            ui.horizontal(|ui| {
                                                let type_color = match p.ptype.as_str() {
                                                    "BAN" => theme.danger,
                                                    "MUTE" => theme.warning,
                                                    "KICK" => theme.danger_hover,
                                                    "WARNING" => theme.warning,
                                                    _ => theme.accent,
                                                };
                                                ui.label(egui::RichText::new(&p.ptype).color(type_color).size(13.0).strong()
                                                    .family(egui::FontFamily::Name("inter".into())));
                                                ui.add_space(8.0);
                                                ui.label(egui::RichText::new(&p.reason).color(theme.text).size(13.0)
                                                    .family(egui::FontFamily::Name("inter".into())));
                                                ui.add_space(8.0);
                                                if current_tab == 0 && p.active {
                                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                        let unpub_id = egui::Id::new(format!("unpub_{}", p.id));
                                                        if danger_button(ui, t("remove", lang), egui::vec2(80.0, 26.0), unpub_id, &theme).clicked() {
                                                            unpublish_action.set(Some(p.id));
                                                        }
                                                    });
                                                }
                                            });
                                            ui.add_space(2.0);
                                            ui.label(egui::RichText::new(format!("{} → {}", p.start, p.end))
                                                .color(theme.text_muted).size(11.0)
                                                .family(egui::FontFamily::Name("inter".into())));
                                        });
                                        ui.add_space(6.0);
                                    }
                                }
                            });
                        } else {
                            ui.add_space(40.0);
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new(t("loading", lang)).color(theme.text_muted).size(14.0)
                                    .family(egui::FontFamily::Name("inter".into())));
                            });
                        }
                    });
            });

        self.punish_reason = reason;
        self.punish_duration = duration;
        if let Some(tab) = switch_tab.get() { self.punish_tab = tab; }
        if close_clicked.get() { self.close_card(); }
        if let Some(ptype) = punish_action.borrow().clone() { self.issue_punishment(&ptype); }
        if let Some(id) = unpublish_action.get() { self.unpublish_punishment(id); }
    }
}

// ---------- EFRAME APP ----------

impl eframe::App for AccessPanelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_messages();
        if self.loading { ctx.request_repaint(); }

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            if self.macro_editor_open {
                self.macro_editor_open = false;
            } else if self.settings_panel.open {
                self.settings_panel.open = false;
            } else if self.show_logout_confirm {
                self.show_logout_confirm = false;
            } else if self.selected.is_some() || self.info.is_some() {
                self.close_card();
            }
        }

        match &mut self.screen {
            AppScreen::Welcome(w) => {
                let theme = self.theme.clone();
                let lang = self.lang;

                if animate_to(&mut self.welcome_anim, 1.0, 0.08) {
                    ctx.request_repaint();
                }

                let result = w.show(ctx, &theme, lang, self.welcome_anim);

                match result {
                    WelcomeResult::Connect(server) => {
                        let id = self.server_list.add(server);
                        self.server_list.set_active(&id);
                        let _ = self.server_list.save();

                        let (pool, rcon_cfg) = build_connections(&self.server_list);
                        self.pool = pool;
                        self.rcon_cfg = rcon_cfg;
                        self.status = "Подключено".into();
                        self.screen = AppScreen::Main;
                    }
                    WelcomeResult::Pending => {}
                }
            }
            AppScreen::AddServer(w) => {
                let theme = self.theme.clone();
                let lang = self.lang;

                if animate_to(&mut self.welcome_anim, 1.0, 0.08) {
                    ctx.request_repaint();
                }

                let result = w.show(ctx, &theme, lang, self.welcome_anim);

                match result {
                    WelcomeResult::Connect(server) => {
                        let _ = self.server_list.add(server);
                        let _ = self.server_list.save();
                        self.screen = AppScreen::Main;
                    }
                    WelcomeResult::Pending => {}
                }
            }
            AppScreen::Main => {
                self.show_main(ctx);

                let theme = self.theme.clone();
                let lang = self.lang;

                let target = if self.settings_panel.open { 1.0 } else { 0.0 };
                if animate_to(&mut self.settings_anim, target, 0.15) {
                    ctx.request_repaint();
                }

                let action = self.settings_panel.show_animated(
                    ctx,
                    &self.server_list,
                    &theme,
                    lang,
                    self.settings_anim,
                );

                match action {
                    PanelAction::Close => { self.settings_panel.open = false; }
                    PanelAction::SwitchServer(id) => { self.switch_server(&id); }
                    PanelAction::AddServer => {
                        self.settings_panel.open = false;
                        self.start_welcome_anim(AppScreen::AddServer(WelcomeScreen::default()));
                    }
                    PanelAction::SwitchTheme(kind) => { self.switch_theme(kind, ctx); }
                    PanelAction::SwitchLang(new_lang) => { self.switch_lang(new_lang); }
                    PanelAction::Logout => { self.logout(); }
                    PanelAction::DeleteServer(_) | PanelAction::None => {}
                }

                let macro_target = if self.macro_editor_open { 1.0 } else { 0.0 };
                if animate_to(&mut self.macro_editor_anim, macro_target, 0.2) {
                    ctx.request_repaint();
                }

                if self.macro_editor_anim > 0.01 {
                    let mut label = self.macro_edit_label.clone();
                    let mut command = self.macro_edit_command.clone();
                    let mut color = self.macro_edit_color.clone();
                    let is_new = self.macro_edit_id.is_none();

                    let result = {
                        let mut editor = MacroEditor {
                            is_new,
                            label: &mut label,
                            command: &mut command,
                            color: &mut color,
                        };
                        editor.show(ctx, &theme, lang, self.macro_editor_anim)
                    };

                    self.macro_edit_label = label;
                    self.macro_edit_command = command;
                    self.macro_edit_color = color;

                    match result {
                        MacroEditorResult::Save => self.save_macro_editor(),
                        MacroEditorResult::Cancel => { self.macro_editor_open = false; }
                        MacroEditorResult::Delete => self.delete_macro(),
                        MacroEditorResult::None => {}
                    }
                }
            }
        }
    }
}

// ---------- MAIN ----------

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([1280.0, 920.0])
        .with_min_inner_size([1100.0, 800.0])
        .with_title("Панель Доступа");

    if let Some(icon) = load_icon() {
        viewport = viewport.with_icon(std::sync::Arc::new(icon));
    }

    let opts = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Панель Доступа",
        opts,
        Box::new(|cc| Ok(Box::new(AccessPanelApp::new(cc)))),
    )
}