// ---------- УТИЛИТЫ ----------
//
// Вспомогательные функции: работа со временем, цветом.
// Здесь нет логики приложения — только «инструменты».

use eframe::egui;

// ---------- ВРЕМЯ ----------

/// Форматирует timestamp в читаемую дату.
/// AdvancedBan и AuthMe хранят время в миллисекундах,
/// но некоторые — в секундах. Функция определяет сама.
pub fn format_ts(ts: i64) -> String {
    let secs = if ts > 10_000_000_000 { ts / 1000 } else { ts };
    let (y, m, d, hh, mm) = time_from_unix(secs);
    format!("{:02}.{:02}.{:04} {:02}:{:02}", d, m, y, hh, mm)
}

/// Конвертирует Unix timestamp (секунды) в дату.
/// Возвращает: (год, месяц, день, часы, минуты).
pub fn time_from_unix(secs: i64) -> (i64, i64, i64, i64, i64) {
    let days = secs / 86400;
    let rem = secs % 86400;
    let hh = rem / 3600;
    let mm = (rem % 3600) / 60;

    let mut y: i64 = 1970;
    let mut d = days;
    loop {
        let leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
        let yd = if leap { 366 } else { 365 };
        if d >= yd {
            d -= yd;
            y += 1;
        } else {
            break;
        }
    }
    let leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
    let months = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut m = 0;
    for (i, &md) in months.iter().enumerate() {
        if d < md {
            m = i as i64 + 1;
            break;
        }
        d -= md;
    }
    if m == 0 {
        m = 12;
    }
    (y, m, d + 1, hh, mm)
}

/// Текущее время в миллисекундах (Unix timestamp * 1000).
/// Используется для AdvancedBan.
pub fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

// ---------- ЦВЕТ ----------

/// Линейная интерполяция между двумя цветами.
/// `t = 0.0` → первый цвет, `t = 1.0` → второй цвет.
/// Используется для плавных hover-эффектов.
pub fn lerp_color(a: egui::Color32, b: egui::Color32, t: f32) -> egui::Color32 {
    let t = t.clamp(0.0, 1.0);
    egui::Color32::from_rgb(
        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
    )
}