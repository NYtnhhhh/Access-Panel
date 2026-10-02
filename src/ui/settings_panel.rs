// ---------- БОКОВАЯ ПАНЕЛЬ НАСТРОЕК ----------

use eframe::egui;

use crate::i18n::{t, Lang};
use crate::servers::ServerList;
use crate::theme::{Theme, ThemeKind};
use crate::ui::{danger_button, smooth_button};

pub struct SettingsPanel {
    pub open: bool,
    pub servers_expanded: bool,
    pub list_anim: f32,
    pub action: PanelAction,
}

#[derive(Clone, Debug, Default)]
pub enum PanelAction {
    #[default]
    None,
    Close,
    SwitchServer(String),
    AddServer,
    DeleteServer(String),
    Logout,
    SwitchTheme(ThemeKind),
    SwitchLang(Lang),
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self {
            open: false,
            servers_expanded: false,
            list_anim: 0.0,
            action: PanelAction::None,
        }
    }
}

impl SettingsPanel {
    pub fn show_animated(
        &mut self,
        ctx: &egui::Context,
        servers: &ServerList,
        theme: &Theme,
        lang: Lang,
        anim: f32,
    ) -> PanelAction {
        self.action = PanelAction::None;

        if anim < 0.005 {
            return PanelAction::None;
        }

        // Анимация раскрытия списка
        let target = if self.servers_expanded { 1.0 } else { 0.0 };
        let speed = 0.15;
        self.list_anim += (target - self.list_anim) * speed;
        if (self.list_anim - target).abs() < 0.001 {
            self.list_anim = target;
        }
        if (self.list_anim - target).abs() > 0.001 {
            ctx.request_repaint();
        }

        let screen = ctx.content_rect();
        let panel_width = 340.0;

        // Затемнение фона — с анимацией
        ctx.layer_painter(egui::LayerId::new(
            egui::Order::Background,
            egui::Id::new("panel_overlay_bg"),
        ))
        .rect_filled(
            screen,
            egui::CornerRadius::ZERO,
            egui::Color32::from_rgba_unmultiplied(0x0a, 0x08, 0x10, (120.0 * anim) as u8),
        );

        // Смещение: 0 = видна, panel_width = скрыта справа
        let offset = (1.0 - anim) * panel_width;

        let panel_rect = egui::Rect::from_min_max(
            egui::pos2(screen.right() - panel_width + offset, screen.top()),
            egui::pos2(screen.right() + offset, screen.bottom()),
        );

        // Тень
        for i in 1..=8 {
            let expand = i as f32 * 3.0 * anim;
            let alpha = ((50.0 - i as f32 * 5.0).max(0.0) * anim) as u8;
            ctx.layer_painter(egui::LayerId::new(
                egui::Order::Background,
                egui::Id::new(format!("panel_shadow_{}", i)),
            ))
            .rect_filled(
                panel_rect.expand2(egui::vec2(expand, 0.0)),
                egui::CornerRadius::ZERO,
                egui::Color32::from_rgba_unmultiplied(0x00, 0x00, 0x00, alpha),
            );
        }

        // Клик вне панели — только когда панель полностью открыта
        if anim > 0.95 {
            let outside_clicked = ctx.input(|i| {
                if let Some(pos) = i.pointer.interact_pos() {
                    if i.pointer.any_click() && !panel_rect.contains(pos) {
                        return true;
                    }
                }
                false
            });
            if outside_clicked {
                self.action = PanelAction::Close;
            }
        }

        egui::Area::new(egui::Id::new("settings_panel_area"))
            .fixed_pos(panel_rect.min)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::NONE
                    .fill(theme.surface)
                    .corner_radius(0.0)
                    .inner_margin(egui::Margin::symmetric(24, 24))
                    .stroke(egui::Stroke::new(1.0_f32, theme.surface_3))
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(panel_width - 48.0, screen.height() - 48.0));
                        ui.set_max_size(egui::vec2(panel_width - 48.0, screen.height() - 48.0));

                        // Прозрачность содержимого
                        ui.set_opacity(anim);

                        // Заголовок
                        ui.horizontal(|ui| {
                            let (r, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                            ui.painter().circle_filled(r.center(), 6.0, theme.accent);
                            ui.add_space(6.0);
                            ui.label(egui::RichText::new(t("settings", lang))
                                .color(theme.text).size(18.0).strong()
                                .family(egui::FontFamily::Name("inter".into())));

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if smooth_button(ui, "X", egui::vec2(32.0, 32.0),
                                                 egui::Id::new("btn_close_panel"),
                                                 theme.surface_2, theme.accent_soft, theme.text).clicked() {
                                    self.action = PanelAction::Close;
                                }
                            });
                        });

                        ui.add_space(20.0);

                        let list_btn_width = panel_width - 48.0;
                        let list_label = if self.servers_expanded {
                            format!("▼  {}", t("servers_list", lang))
                        } else {
                            format!("▶  {}", t("servers_list", lang))
                        };

                        if smooth_button(ui, &list_label,
                                         egui::vec2(list_btn_width, 44.0),
                                         egui::Id::new("btn_servers_list"),
                                         theme.surface_2, theme.accent_soft, theme.text).clicked() {
                            self.servers_expanded = !self.servers_expanded;
                        }

                        if self.list_anim > 0.01 {
                            ui.add_space(10.0 * self.list_anim);

                            egui::Frame::NONE
                                .fill(theme.surface_2)
                                .corner_radius(10.0)
                                .inner_margin(10.0)
                                .show(ui, |ui| {
                                    ui.set_min_width(list_btn_width - 20.0);
                                    ui.set_opacity(self.list_anim);

                                    if servers.servers.is_empty() {
                                        ui.label(egui::RichText::new(t("no_servers", lang))
                                            .color(theme.text_muted).size(12.0)
                                            .family(egui::FontFamily::Name("inter".into())));
                                    } else {
                                        for server in &servers.servers {
                                            let is_active = servers.active_id.as_deref()
                                                == Some(&server.id);
                                            let label = if is_active {
                                                format!("●  {}", server.name)
                                            } else {
                                                format!("○  {}", server.name)
                                            };

                                            let text_color = if is_active { theme.btn_text } else { theme.text };
                                            let bg_color = if is_active { theme.accent } else { theme.surface_3 };
                                            let hover_color = if is_active { theme.accent_bright } else { theme.accent_soft };

                                            if smooth_button(ui, &label,
                                                             egui::vec2(list_btn_width - 20.0, 32.0),
                                                             egui::Id::new(format!("server_{}", server.id)),
                                                             bg_color, hover_color, text_color).clicked() {
                                                self.action = PanelAction::SwitchServer(server.id.clone());
                                                self.servers_expanded = false;
                                            }
                                            ui.add_space(4.0);
                                        }
                                    }

                                    ui.add_space(6.0);
                                    ui.separator();
                                    ui.add_space(6.0);

                                    if smooth_button(ui, t("add_server", lang),
                                                     egui::vec2(list_btn_width - 20.0, 36.0),
                                                     egui::Id::new("btn_add_server"),
                                                     theme.surface_3, theme.accent_soft, theme.text).clicked() {
                                        self.action = PanelAction::AddServer;
                                    }
                                });
                        }

                        ui.add_space(20.0);

                        // Темы
                        ui.label(egui::RichText::new(t("theme", lang))
                            .color(theme.text).size(13.0).strong()
                            .family(egui::FontFamily::Name("inter".into())));
                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            let dark_selected = theme.kind == ThemeKind::Dark;
                            let dark_bg = if dark_selected { theme.accent } else { theme.surface_2 };
                            let dark_fg = if dark_selected { theme.btn_text } else { theme.text };

                            if smooth_button(ui, t("theme_dark", lang),
                                             egui::vec2((list_btn_width - 10.0) / 2.0, 36.0),
                                             egui::Id::new("btn_theme_dark"),
                                             dark_bg, theme.accent_soft, dark_fg).clicked() {
                                self.action = PanelAction::SwitchTheme(ThemeKind::Dark);
                            }

                            ui.add_space(10.0);

                            let light_selected = theme.kind == ThemeKind::Light;
                            let light_bg = if light_selected { theme.accent } else { theme.surface_2 };
                            let light_fg = if light_selected { theme.btn_text } else { theme.text };

                            if smooth_button(ui, t("theme_light", lang),
                                             egui::vec2((list_btn_width - 10.0) / 2.0, 36.0),
                                             egui::Id::new("btn_theme_light"),
                                             light_bg, theme.accent_soft, light_fg).clicked() {
                                self.action = PanelAction::SwitchTheme(ThemeKind::Light);
                            }
                        });

                        ui.add_space(16.0);

                        // Язык
                        ui.label(egui::RichText::new(t("language", lang))
                            .color(theme.text).size(13.0).strong()
                            .family(egui::FontFamily::Name("inter".into())));
                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            let ru_selected = lang == Lang::Ru;
                            let ru_bg = if ru_selected { theme.accent } else { theme.surface_2 };
                            let ru_fg = if ru_selected { theme.btn_text } else { theme.text };

                            if smooth_button(ui, "RU",
                                             egui::vec2((list_btn_width - 10.0) / 2.0, 36.0),
                                             egui::Id::new("btn_lang_ru"),
                                             ru_bg, theme.accent_soft, ru_fg).clicked() {
                                self.action = PanelAction::SwitchLang(Lang::Ru);
                            }

                            ui.add_space(10.0);

                            let en_selected = lang == Lang::En;
                            let en_bg = if en_selected { theme.accent } else { theme.surface_2 };
                            let en_fg = if en_selected { theme.btn_text } else { theme.text };

                            if smooth_button(ui, "EN",
                                             egui::vec2((list_btn_width - 10.0) / 2.0, 36.0),
                                             egui::Id::new("btn_lang_en"),
                                             en_bg, theme.accent_soft, en_fg).clicked() {
                                self.action = PanelAction::SwitchLang(Lang::En);
                            }
                        });

                        // Прижимаем к низу
                        let used = ui.min_rect().height();
                        let available = screen.height() - 48.0;
                        if available > used + 80.0 {
                            ui.add_space(available - used - 80.0);
                        } else {
                            ui.add_space(20.0);
                        }

                        ui.separator();
                        ui.add_space(12.0);

                        ui.vertical_centered(|ui| {
                            if danger_button(ui, t("logout", lang),
                                             egui::vec2(list_btn_width, 44.0),
                                             egui::Id::new("btn_logout"),
                                             theme).clicked() {
                                self.action = PanelAction::Logout;
                            }
                        });
                    });
            });

        self.action.clone()
    }
}