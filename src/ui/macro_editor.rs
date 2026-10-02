// ---------- РЕДАКТОР МАКРОСА ----------

use eframe::egui;

use crate::i18n::{t, Lang};
use crate::macros::{color_presets, parse_hex_color};
use crate::theme::Theme;
use crate::ui::{danger_button, gradient_button, smooth_button};

pub enum MacroEditorResult {
    None,
    Cancel,
    Save,
    Delete,
}

pub struct MacroEditor<'a> {
    pub is_new: bool,
    pub label: &'a mut String,
    pub command: &'a mut String,
    pub color: &'a mut String,
}

impl<'a> MacroEditor<'a> {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        theme: &Theme,
        lang: Lang,
        anim: f32,
    ) -> MacroEditorResult {
        let mut result = MacroEditorResult::None;

        if anim < 0.005 {
            return result;
        }

        let screen = ctx.content_rect();

        ctx.layer_painter(egui::LayerId::new(
            egui::Order::Background,
            egui::Id::new("macro_editor_overlay"),
        ))
        .rect_filled(
            screen,
            egui::CornerRadius::ZERO,
            egui::Color32::from_rgba_unmultiplied(0x0a, 0x08, 0x10, (180.0 * anim) as u8),
        );

        let scale = 0.9 + 0.1 * anim;
        let dialog_w = 520.0 * scale;
        let dialog_h = 540.0 * scale;
        let dialog_rect = egui::Rect::from_center_size(
            screen.center(),
            egui::vec2(dialog_w, dialog_h),
        );

        egui::Area::new(egui::Id::new("macro_editor_area"))
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

                        let title = if self.is_new {
                            t("macro_new", lang)
                        } else {
                            t("macro_edit", lang)
                        };
                        ui.horizontal(|ui| {
                            let (r, _) = ui.allocate_exact_size(
                                egui::vec2(14.0, 14.0),
                                egui::Sense::hover(),
                            );
                            ui.painter().circle_filled(r.center(), 6.0, theme.accent);
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(title)
                                    .color(theme.text)
                                    .size(20.0)
                                    .strong()
                                    .family(egui::FontFamily::Name("inter".into())),
                            );
                        });

                        ui.add_space(20.0);

                        ui.label(
                            egui::RichText::new(t("macro_name", lang))
                                .color(theme.text_muted)
                                .size(12.0)
                                .family(egui::FontFamily::Name("inter".into())),
                        );
                        ui.add_space(4.0);
                        ui.add_sized(
                            [dialog_w - 56.0, 40.0],
                            egui::TextEdit::singleline(self.label)
                                .hint_text(t("macro_name_hint", lang))
                                .text_color(theme.text)
                                .margin(egui::Margin::symmetric(12, 10)),
                        );

                        ui.add_space(14.0);

                        ui.label(
                            egui::RichText::new(t("macro_command", lang))
                                .color(theme.text_muted)
                                .size(12.0)
                                .family(egui::FontFamily::Name("inter".into())),
                        );
                        ui.add_space(4.0);
                        ui.add_sized(
                            [dialog_w - 56.0, 40.0],
                            egui::TextEdit::singleline(self.command)
                                .hint_text(t("macro_command_hint", lang))
                                .text_color(theme.text)
                                .margin(egui::Margin::symmetric(12, 10)),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(t("macro_command_hint2", lang))
                                .color(theme.hint)
                                .size(11.0)
                                .family(egui::FontFamily::Name("inter".into())),
                        );

                        ui.add_space(14.0);

                        ui.label(
                            egui::RichText::new(t("macro_color", lang))
                                .color(theme.text_muted)
                                .size(12.0)
                                .family(egui::FontFamily::Name("inter".into())),
                        );
                        ui.add_space(8.0);

                        let current_color = parse_hex_color(self.color);

                        ui.horizontal_wrapped(|ui| {
                            for (name, hex) in color_presets() {
                                let col = parse_hex_color(hex);
                                let selected = self.color.to_lowercase() == hex.to_lowercase();

                                let (rect, resp) = ui.allocate_exact_size(
                                    egui::vec2(50.0, 36.0),
                                    egui::Sense::click(),
                                );
                                let hover_lvl = ui.ctx().animate_bool_with_time(
                                    egui::Id::new(format!("color_preset_{}", hex)),
                                    resp.hovered(),
                                    0.2,
                                );

                                let painter = ui.painter();
                                painter.rect_filled(rect, egui::CornerRadius::same(8), col);

                                if selected {
                                    painter.rect_stroke(
                                        rect,
                                        egui::CornerRadius::same(8),
                                        egui::Stroke::new(3.0_f32, theme.text),
                                        egui::StrokeKind::Inside,
                                    );
                                } else if hover_lvl > 0.05 {
                                    painter.rect_stroke(
                                        rect,
                                        egui::CornerRadius::same(8),
                                        egui::Stroke::new(
                                            2.0_f32,
                                            egui::Color32::from_rgba_unmultiplied(
                                                theme.text.r(),
                                                theme.text.g(),
                                                theme.text.b(),
                                                (200.0 * hover_lvl) as u8,
                                            ),
                                        ),
                                        egui::StrokeKind::Inside,
                                    );
                                }

                                if resp.clicked() {
                                    *self.color = hex.to_string();
                                }
                                if resp.hovered() {
                                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                    resp.on_hover_text(name);
                                }
                            }
                        });

                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(t("macro_custom", lang))
                                    .color(theme.text_muted)
                                    .size(12.0)
                                    .family(egui::FontFamily::Name("inter".into())),
                            );
                            ui.add_sized(
                                [140.0, 32.0],
                                egui::TextEdit::singleline(self.color)
                                    .hint_text("#a78bfa")
                                    .text_color(theme.text)
                                    .margin(egui::Margin::symmetric(10, 6)),
                            );

                            ui.add_space(10.0);

                            let (rect, _) = ui.allocate_exact_size(
                                egui::vec2(60.0, 32.0),
                                egui::Sense::hover(),
                            );
                            ui.painter().rect_filled(
                                rect,
                                egui::CornerRadius::same(6),
                                current_color,
                            );
                        });

                        ui.add_space(20.0);

                        ui.horizontal(|ui| {
                            if gradient_button(
                                ui,
                                t("macro_save", lang),
                                egui::vec2(160.0, 44.0),
                                egui::Id::new("btn_macro_save"),
                                theme,
                            )
                            .clicked()
                            {
                                result = MacroEditorResult::Save;
                            }

                            ui.add_space(10.0);

                            if smooth_button(
                                ui,
                                t("cancel", lang),
                                egui::vec2(140.0, 44.0),
                                egui::Id::new("btn_macro_cancel"),
                                theme.surface_2,
                                theme.accent_soft,
                                theme.text,
                            )
                            .clicked()
                            {
                                result = MacroEditorResult::Cancel;
                            }

                            if !self.is_new {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if danger_button(
                                            ui,
                                            t("macro_delete", lang),
                                            egui::vec2(140.0, 44.0),
                                            egui::Id::new("btn_macro_delete"),
                                            theme,
                                        )
                                        .clicked()
                                        {
                                            result = MacroEditorResult::Delete;
                                        }
                                    },
                                );
                            }
                        });
                    });
            });

        result
    }
}