// ---------- UI МОДУЛЬ ----------
//
// Объединяет все UI-компоненты приложения.

pub mod buttons;
pub mod fonts;
pub mod welcome;
pub mod settings_panel;
pub mod macro_editor;

pub use buttons::*;
pub use fonts::*;
pub use welcome::*;
pub use settings_panel::*;
pub use macro_editor::{MacroEditor, MacroEditorResult};