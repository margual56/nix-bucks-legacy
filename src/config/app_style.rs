use eframe::egui::Color32;
use serde::{Deserialize, Serialize};

use crate::color::ColorHex;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStyle {
    #[serde(with = "crate::serde_helpers::color")]
    pub background: Color32,
    #[serde(with = "crate::serde_helpers::color")]
    pub foreground: Color32,

    #[serde(with = "crate::serde_helpers::color")]
    pub circle_focus: Color32,
    #[serde(with = "crate::serde_helpers::color")]
    pub circle_short_break: Color32,
    #[serde(with = "crate::serde_helpers::color")]
    pub circle_long_break: Color32,

    #[serde(with = "crate::serde_helpers::color")]
    pub rounds,
}
