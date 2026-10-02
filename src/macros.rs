// ---------- КНОПКИ-МАКРОСЫ ----------
//
// Кастомные кнопки с RCON-командами.
// Хранятся в buttons.toml.

use eframe::egui;
use serde::{Deserialize, Serialize};

/// Одна кнопка-макрос.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MacroButton {
    pub id: String,
    pub label: String,
    pub command: String,
    /// Цвет в hex: "#a78bfa"
    pub color: String,
}

/// Список кнопок.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MacroList {
    #[serde(default)]
    pub buttons: Vec<MacroButton>,
}

const MACROS_FILE: &str = "buttons.toml";

impl MacroList {
    pub fn load() -> Self {
        match std::fs::read_to_string(MACROS_FILE) {
            Ok(s) => toml::from_str(&s).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let s = toml::to_string_pretty(self)
            .map_err(|e| format!("Сериализация: {}", e))?;
        std::fs::write(MACROS_FILE, s)
            .map_err(|e| format!("Запись buttons.toml: {}", e))
    }

    pub fn add(&mut self, label: String, command: String, color: String) -> String {
        let id = format!("macro_{}", now_millis());
        self.buttons.push(MacroButton {
            id: id.clone(),
            label,
            command,
            color,
        });
        id
    }

    pub fn remove(&mut self, id: &str) {
        self.buttons.retain(|b| b.id != id);
    }

    pub fn update(&mut self, id: &str, label: String, command: String, color: String) {
        if let Some(b) = self.buttons.iter_mut().find(|b| b.id == id) {
            b.label = label;
            b.command = command;
            b.color = color;
        }
    }
}

fn now_millis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

/// Парсит hex-цвет "#rrggbb" → Color32.
pub fn parse_hex_color(s: &str) -> egui::Color32 {
    let s = s.trim_start_matches('#');
    if s.len() != 6 {
        return egui::Color32::from_rgb(0xa7, 0x8b, 0xfa);
    }
    let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0xa7);
    let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0x8b);
    let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0xfa);
    egui::Color32::from_rgb(r, g, b)
}

/// Пресеты цветов — для редактора.
pub fn color_presets() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Фиолетовый", "#a78bfa"),
        ("Синий",      "#007aff"),
        ("Зелёный",    "#34c759"),
        ("Жёлтый",     "#ffcc00"),
        ("Оранжевый",  "#ff9500"),
        ("Красный",    "#ff3b30"),
        ("Розовый",    "#ff2d55"),
        ("Бирюзовый",  "#5ac8fa"),
    ]
}