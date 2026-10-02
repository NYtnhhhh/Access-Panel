// ---------- КНОПКИ ----------
//
// Три вида кнопок с поддержкой тем.

use eframe::egui;

use crate::theme::Theme;
use crate::utils::lerp_color;

/// Главная кнопка — акцентный градиент, свечение при наведении.
pub fn gradient_button(
    ui: &mut egui::Ui,
    text: &str,
    size: egui::Vec2,
    id: egui::Id,
    theme: &Theme,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let hovered = response.hovered();
    let lvl = ui.ctx().animate_bool_with_time(id, hovered, 0.35);
    let painter = ui.painter();
    let radius = 14.0;

    if lvl > 0.005 {
        for i in 1..=8 {
            let expand = i as f32 * 2.2 * lvl;
            let alpha = ((36.0 - i as f32 * 4.0) * lvl).max(0.0) as u8;
            painter.rect_filled(
                rect.expand(expand),
                egui::CornerRadius::same((radius as u8 + i as u8 * 2).min(255)),
                egui::Color32::from_rgba_unmultiplied(
                    theme.accent.r(), theme.accent.g(), theme.accent.b(), alpha
                ),
            );
        }
    }

    let color = lerp_color(theme.accent, theme.accent_bright, lvl);
    painter.rect_filled(rect, egui::CornerRadius::same(radius as u8), color);
    painter.rect_stroke(
        rect,
        egui::CornerRadius::same(radius as u8),
        egui::Stroke::new(0.6_f32, egui::Color32::from_rgba_unmultiplied(0xff, 0xff, 0xff, 180)),
        egui::StrokeKind::Inside,
    );

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::new(15.0, egui::FontFamily::Name("inter".into())),
        theme.btn_text,
    );

    response
}

/// Нейтральная кнопка — серая, плавный hover.
pub fn smooth_button(
    ui: &mut egui::Ui,
    text: &str,
    size: egui::Vec2,
    id: egui::Id,
    fill: egui::Color32,
    hover_fill: egui::Color32,
    text_color: egui::Color32,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let lvl = ui.ctx().animate_bool_with_time(id, response.hovered(), 0.3);
    let painter = ui.painter();
    let radius = 10.0;

    let bg_col = lerp_color(fill, hover_fill, lvl);
    painter.rect_filled(rect, egui::CornerRadius::same(radius as u8), bg_col);
    painter.rect_stroke(
        rect,
        egui::CornerRadius::same(radius as u8),
        egui::Stroke::new(0.6_f32, egui::Color32::from_rgba_unmultiplied(0xff, 0xff, 0xff, 120)),
        egui::StrokeKind::Inside,
    );

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::new(14.0, egui::FontFamily::Name("inter".into())),
        text_color,
    );

    response
}

/// Опасная кнопка — красная, со свечением при hover.
pub fn danger_button(
    ui: &mut egui::Ui,
    text: &str,
    size: egui::Vec2,
    id: egui::Id,
    theme: &Theme,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let lvl = ui.ctx().animate_bool_with_time(id, response.hovered(), 0.3);
    let painter = ui.painter();
    let radius = 10.0;

    if lvl > 0.005 {
        for i in 1..=6 {
            let expand = i as f32 * 2.0 * lvl;
            let alpha = ((30.0 - i as f32 * 4.0) * lvl).max(0.0) as u8;
            painter.rect_filled(
                rect.expand(expand),
                egui::CornerRadius::same((radius as u8 + i as u8 * 2).min(255)),
                egui::Color32::from_rgba_unmultiplied(
                    theme.danger.r(), theme.danger.g(), theme.danger.b(), alpha
                ),
            );
        }
    }

    let bg_col = lerp_color(theme.danger, theme.danger_hover, lvl);
    painter.rect_filled(rect, egui::CornerRadius::same(radius as u8), bg_col);
    painter.rect_stroke(
        rect,
        egui::CornerRadius::same(radius as u8),
        egui::Stroke::new(0.6_f32, egui::Color32::from_rgba_unmultiplied(0xff, 0xff, 0xff, 180)),
        egui::StrokeKind::Inside,
    );

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::new(14.0, egui::FontFamily::Name("inter".into())),
        egui::Color32::WHITE,
    );

    response
}