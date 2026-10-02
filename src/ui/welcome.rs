// ---------- ЭКРАН "ДОБРО ПОЖАЛОВАТЬ" ----------

use eframe::egui;

use crate::i18n::{t, Lang};
use crate::servers::{MysqlConfig, RconData, Server};
use crate::theme::Theme;
use crate::ui::gradient_button;

pub enum WelcomeResult {
    Connect(Server),
    Pending,
}

pub struct WelcomeScreen {
    pub server_name: String,
    pub mysql_host: String,
    pub mysql_port: String,
    pub mysql_user: String,
    pub mysql_password: String,
    pub mysql_database: String,
    pub rcon_host: String,
    pub rcon_port: String,
    pub rcon_password: String,
    pub remember: bool,
    pub error: String,
}

impl Default for WelcomeScreen {
    fn default() -> Self {
        Self {
            server_name: String::new(),
            mysql_host: String::new(),
            mysql_port: "3306".into(),
            mysql_user: String::new(),
            mysql_password: String::new(),
            mysql_database: String::new(),
            rcon_host: String::new(),
            rcon_port: "25575".into(),
            rcon_password: String::new(),
            remember: true,
            error: String::new(),
        }
    }
}

impl WelcomeScreen {
    pub fn try_build_server(&self, lang: Lang) -> Result<Server, String> {
        if self.mysql_host.trim().is_empty() {
            return Err(t("err_mysql_host", lang).into());
        }
        if self.mysql_user.trim().is_empty() {
            return Err(t("err_mysql_user", lang).into());
        }
        if self.mysql_database.trim().is_empty() {
            return Err(t("err_mysql_db", lang).into());
        }

        Ok(Server {
            id: String::new(),
            name: self.server_name.clone(),
            remember: self.remember,
            mysql: MysqlConfig {
                host: self.mysql_host.trim().to_string(),
                port: self.mysql_port.trim().parse().unwrap_or(3306),
                user: self.mysql_user.trim().to_string(),
                password: self.mysql_password.clone(),
                database: self.mysql_database.trim().to_string(),
            },
            rcon: RconData {
                host: self.rcon_host.trim().to_string(),
                port: self.rcon_port.trim().parse().unwrap_or(25575),
                password: self.rcon_password.clone(),
            },
        })
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        theme: &Theme,
        lang: Lang,
        anim: f32,
    ) -> WelcomeResult {
        let mut result = WelcomeResult::Pending;

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme.bg).inner_margin(egui::Margin::symmetric(80, 40)))
            .show(ctx, |ui| {
                ui.set_opacity(anim);

                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);

                    ui.horizontal(|ui| {
                        let total_width = 280.0;
                        let avail = ui.available_width();
                        ui.add_space((avail - total_width) / 2.0);

                        let (rect, _) = ui.allocate_exact_size(egui::vec2(14.0, 30.0), egui::Sense::hover());
                        ui.painter().circle_filled(rect.center(), 8.0, theme.accent);
                        ui.painter().circle_filled(rect.center(), 14.0,
                            egui::Color32::from_rgba_unmultiplied(
                                theme.accent.r(), theme.accent.g(), theme.accent.b(), 45));

                        ui.add_space(8.0);
                        ui.label(egui::RichText::new(t("app_title", lang))
                            .color(theme.text).size(28.0).strong()
                            .family(egui::FontFamily::Name("inter".into())));
                    });

                    ui.add_space(20.0);
                    ui.label(egui::RichText::new(t("welcome", lang))
                        .color(theme.text).size(22.0).strong()
                        .family(egui::FontFamily::Name("inter".into())));
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(t("welcome_desc", lang))
                        .color(theme.text_muted).size(13.0)
                        .family(egui::FontFamily::Name("inter".into())));

                    ui.add_space(24.0);
                });

                let form_width = 600.0;
                let avail = ui.available_width();

                ui.allocate_ui_with_layout(
                    egui::vec2(avail, 0.0),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        egui::Frame::NONE
                            .fill(theme.surface)
                            .corner_radius(16.0)
                            .inner_margin(28.0)
                            .stroke(egui::Stroke::new(1.0_f32, theme.surface_3))
                            .show(ui, |ui| {
                                ui.set_width(form_width);

                                field_label(ui, t("server_name", lang), theme);
                                ui.add_sized(
                                    [form_width, 34.0],
                                    egui::TextEdit::singleline(&mut self.server_name)
                                        .hint_text(t("server_name_hint", lang))
                                        .text_color(theme.text)
                                        .margin(egui::Margin::symmetric(12, 8))
                                );
                                ui.add_space(14.0);

                                ui.label(egui::RichText::new("MySQL")
                                    .color(theme.accent).size(13.0).strong()
                                    .family(egui::FontFamily::Name("inter".into())));
                                ui.add_space(6.0);

                                field_row(ui, t("host", lang), &mut self.mysql_host, "mysql.example.com", form_width, false, theme);
                                field_row_small(ui, t("port", lang), &mut self.mysql_port, "3306", form_width, theme);
                                field_row(ui, t("user", lang), &mut self.mysql_user, "user", form_width, false, theme);
                                field_row(ui, t("password", lang), &mut self.mysql_password, "password", form_width, true, theme);
                                field_row(ui, t("database", lang), &mut self.mysql_database, "database", form_width, false, theme);

                                ui.add_space(14.0);

                                ui.label(egui::RichText::new("RCON")
                                    .color(theme.accent).size(13.0).strong()
                                    .family(egui::FontFamily::Name("inter".into())));
                                ui.add_space(6.0);

                                field_row(ui, t("host", lang), &mut self.rcon_host, "play.myserver.com", form_width, false, theme);
                                field_row_small(ui, t("port", lang), &mut self.rcon_port, "25575", form_width, theme);
                                field_row(ui, t("password", lang), &mut self.rcon_password, "rcon_password", form_width, true, theme);

                                ui.add_space(14.0);

                                ui.checkbox(&mut self.remember,
                                    egui::RichText::new(t("remember", lang))
                                        .color(theme.text).size(13.0)
                                        .family(egui::FontFamily::Name("inter".into())));

                                ui.add_space(10.0);

                                if !self.error.is_empty() {
                                    ui.label(egui::RichText::new(&self.error)
                                        .color(theme.danger).size(12.0)
                                        .family(egui::FontFamily::Name("inter".into())));
                                    ui.add_space(8.0);
                                }

                                ui.vertical_centered(|ui| {
                                    if gradient_button(ui, t("connect", lang),
                                                       egui::vec2(220.0, 46.0),
                                                       egui::Id::new("btn_connect"),
                                                       theme).clicked() {
                                        match self.try_build_server(lang) {
                                            Ok(server) => {
                                                self.error.clear();
                                                result = WelcomeResult::Connect(server);
                                            }
                                            Err(e) => {
                                                self.error = e;
                                            }
                                        }
                                    }
                                });
                            });

                        ui.add_space(20.0);
                        ui.label(egui::RichText::new(t("passwords_warning", lang))
                            .color(theme.text_muted).size(10.0)
                            .family(egui::FontFamily::Name("inter".into())));
                    },
                );
            });

        result
    }
}

fn field_label(ui: &mut egui::Ui, label: &str, theme: &Theme) {
    ui.label(egui::RichText::new(label)
        .color(theme.text_muted).size(12.0)
        .family(egui::FontFamily::Name("inter".into())));
}

fn field_row(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut String,
    hint: &str,
    width: f32,
    password: bool,
    theme: &Theme,
) {
    ui.horizontal(|ui| {
        ui.add_sized(
            [120.0, 34.0],
            egui::Label::new(
                egui::RichText::new(label)
                    .color(theme.text_muted).size(12.0)
                    .family(egui::FontFamily::Name("inter".into()))
            )
        );
        ui.add_sized(
            [width - 130.0, 34.0],
            egui::TextEdit::singleline(value)
                .hint_text(hint)
                .password(password)
                .text_color(theme.text)
                .margin(egui::Margin::symmetric(12, 8))
        );
    });
    ui.add_space(6.0);
}

fn field_row_small(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut String,
    hint: &str,
    width: f32,
    theme: &Theme,
) {
    ui.horizontal(|ui| {
        ui.add_sized(
            [120.0, 34.0],
            egui::Label::new(
                egui::RichText::new(label)
                    .color(theme.text_muted).size(12.0)
                    .family(egui::FontFamily::Name("inter".into()))
            )
        );
        ui.add_sized(
            [100.0, 34.0],
            egui::TextEdit::singleline(value)
                .hint_text(hint)
                .text_color(theme.text)
                .margin(egui::Margin::symmetric(12, 8))
        );
        ui.add_space(width - 230.0);
    });
    ui.add_space(6.0);
}