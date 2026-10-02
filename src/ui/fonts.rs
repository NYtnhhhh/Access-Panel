// ---------- ШРИФТЫ И ИКОНКА ----------
//
// Загрузка шрифта Inter и иконки окна.

use eframe::egui;

/// Настраивает шрифт Inter как основной для всего приложения.
pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "inter".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/InterVariable.ttf")).into(),
    );

    fonts.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "inter".to_owned());

    fonts.families
        .entry(egui::FontFamily::Name("inter".into()))
        .or_default()
        .insert(0, "inter".to_owned());

    ctx.set_fonts(fonts);
}

/// Загружает иконку окна из `assets/icon.png`.
/// Автоматически масштабирует до 256×256.
pub fn load_icon() -> Option<egui::IconData> {
    let bytes = include_bytes!("../../assets/icon.png");
    let img = image::load_from_memory(bytes).ok()?;
    let resized = img.resize_exact(256, 256, image::imageops::FilterType::Lanczos3);
    let rgba = resized.to_rgba8();
    let (w, h) = rgba.dimensions();
    Some(egui::IconData {
        rgba: rgba.into_raw(),
        width: w,
        height: h,
    })
}