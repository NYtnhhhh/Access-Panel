// ---------- ТЕМЫ ----------
//
// Две темы: Dark (тёмная) и Light (светлая).
// Структура Theme хранит все цвета приложения.
// Передаётся во все UI-функции.

use eframe::egui;

// ---------- ВИД ТЕМЫ ----------

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ThemeKind {
    Dark,
    Light,
}

// ---------- СТРУКТУРА ТЕМЫ ----------

#[derive(Clone, Debug)]
pub struct Theme {
    pub kind: ThemeKind,

    // Фон
    pub bg: egui::Color32,
    pub surface: egui::Color32,
    pub surface_2: egui::Color32,
    pub surface_3: egui::Color32,

    // Текст
    pub text: egui::Color32,
    pub text_muted: egui::Color32,
    pub hint: egui::Color32,

    // Акцент (фиолетовый или синий)
    pub accent: egui::Color32,
    pub accent_bright: egui::Color32,
    pub accent_soft: egui::Color32,

    // Статусы
    pub success: egui::Color32,
    pub danger: egui::Color32,
    pub danger_hover: egui::Color32,
    pub warning: egui::Color32,

    // Текст на кнопках
    pub btn_text: egui::Color32,
}

impl Theme {
    /// Тёмная тема — фиолетовый акцент (текущая).
    pub fn dark() -> Self {
        Self {
            kind: ThemeKind::Dark,

            bg: egui::Color32::from_rgb(0x12, 0x10, 0x16),
            surface: egui::Color32::from_rgb(0x1a, 0x17, 0x20),
            surface_2: egui::Color32::from_rgb(0x22, 0x1e, 0x2b),
            surface_3: egui::Color32::from_rgb(0x2a, 0x24, 0x35),

            text: egui::Color32::from_rgb(0xf5, 0xf0, 0xff),
            text_muted: egui::Color32::from_rgb(0x8a, 0x84, 0x94),
            hint: egui::Color32::from_rgb(0x5c, 0x56, 0x66),

            accent: egui::Color32::from_rgb(0xa7, 0x8b, 0xfa),
            accent_bright: egui::Color32::from_rgb(0xc4, 0xb5, 0xfd),
            accent_soft: egui::Color32::from_rgb(0x3d, 0x2b, 0x66),

            success: egui::Color32::from_rgb(0x6e, 0xe7, 0xb7),
            danger: egui::Color32::from_rgb(0xef, 0x44, 0x44),
            danger_hover: egui::Color32::from_rgb(0xf8, 0x71, 0x71),
            warning: egui::Color32::from_rgb(0xfb, 0xbf, 0x24),

            btn_text: egui::Color32::from_rgb(0x12, 0x10, 0x16),
        }
    }

    /// Светлая тема — синий акцент (как iOS).
    pub fn light() -> Self {
        Self {
            kind: ThemeKind::Light,

            bg: egui::Color32::from_rgb(0xf5, 0xf5, 0xf7),
            surface: egui::Color32::from_rgb(0xff, 0xff, 0xff),
            surface_2: egui::Color32::from_rgb(0xf0, 0xf0, 0xf2),
            surface_3: egui::Color32::from_rgb(0xe5, 0xe5, 0xea),

            text: egui::Color32::from_rgb(0x1d, 0x1d, 0x1f),
            text_muted: egui::Color32::from_rgb(0x6e, 0x6e, 0x73),
            hint: egui::Color32::from_rgb(0xa1, 0xa1, 0xa6),

            accent: egui::Color32::from_rgb(0x00, 0x7a, 0xff),
            accent_bright: egui::Color32::from_rgb(0x33, 0x95, 0xff),
            accent_soft: egui::Color32::from_rgb(0xd6, 0xe8, 0xff),

            success: egui::Color32::from_rgb(0x34, 0xc7, 0x59),
            danger: egui::Color32::from_rgb(0xff, 0x3b, 0x30),
            danger_hover: egui::Color32::from_rgb(0xff, 0x6b, 0x63),
            warning: egui::Color32::from_rgb(0xff, 0x9f, 0x0a),

            btn_text: egui::Color32::from_rgb(0xff, 0xff, 0xff),
        }
    }

    /// Возвращает тему по виду.
    pub fn from_kind(kind: ThemeKind) -> Self {
        match kind {
            ThemeKind::Dark => Self::dark(),
            ThemeKind::Light => Self::light(),
        }
    }

    /// Применяет цвета к визуальным настройкам egui.
    pub fn apply(&self, ctx: &egui::Context) {
        let mut visuals = match self.kind {
            ThemeKind::Dark => egui::Visuals::dark(),
            ThemeKind::Light => egui::Visuals::light(),
        };

        visuals.panel_fill = self.bg;
        visuals.window_fill = self.surface;
        visuals.extreme_bg_color = self.surface_2;
        visuals.faint_bg_color = self.surface;
        visuals.override_text_color = Some(self.text);
        visuals.weak_text_color = Some(self.hint);

        visuals.selection.bg_fill = egui::Color32::from_rgba_unmultiplied(
            self.accent.r(), self.accent.g(), self.accent.b(), 90
        );
        visuals.selection.stroke = egui::Stroke::new(1.5_f32, self.accent_bright);

        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0_f32, self.surface_3);
        visuals.widgets.inactive.bg_fill = self.surface_2;
        visuals.widgets.inactive.weak_bg_fill = self.surface_2;
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0_f32, self.surface_3);
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0_f32, self.text);

        visuals.widgets.hovered.bg_fill = self.accent_soft;
        visuals.widgets.hovered.weak_bg_fill = self.accent_soft;
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.5_f32, self.accent);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0_f32, self.accent_bright);

        visuals.widgets.active.bg_fill = self.accent;
        visuals.widgets.active.weak_bg_fill = self.accent;
        visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0_f32, self.accent_bright);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0_f32, self.btn_text);

        visuals.widgets.open.bg_fill = self.surface_3;
        visuals.widgets.open.bg_stroke = egui::Stroke::new(1.0_f32, self.accent);

        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(14.0, 14.0);
        style.spacing.button_padding = egui::vec2(20.0, 12.0);
        style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(12);
        style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(12);
        style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(12);
        style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(12);
        style.visuals.extreme_bg_color = self.surface_2;
        style.visuals.weak_text_color = Some(self.hint);
        ctx.set_style(style);
    }
}

// ---------- СОХРАНЕНИЕ ТЕМЫ ----------

const THEME_FILE: &str = "theme.txt";

/// Читает тему из файла. Если файла нет — Dark.
pub fn load_theme_kind() -> ThemeKind {
    match std::fs::read_to_string(THEME_FILE) {
        Ok(s) => match s.trim() {
            "light" => ThemeKind::Light,
            _ => ThemeKind::Dark,
        },
        Err(_) => ThemeKind::Dark,
    }
}

/// Сохраняет тему в файл.
pub fn save_theme_kind(kind: ThemeKind) -> Result<(), String> {
    let s = match kind {
        ThemeKind::Dark => "dark",
        ThemeKind::Light => "light",
    };
    std::fs::write(THEME_FILE, s)
        .map_err(|e| format!("Запись theme.txt: {}", e))
}