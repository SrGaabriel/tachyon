use std::ops::Not;
use serde::{Deserialize, Serialize};
use crate::game::identifier::NamespaceId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Style {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub bold: bool,
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub italic: bool,
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub underlined: bool,
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub strikethrough: bool,
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub obfuscated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<NamespaceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub value: String
}

impl Color {
    pub fn black() -> Self {
        Color { value: "black".to_string() }
    }

    pub fn dark_blue() -> Self {
        Color { value: "dark_blue".to_string() }
    }

    pub fn dark_green() -> Self {
        Color { value: "dark_green".to_string() }
    }

    pub fn dark_aqua() -> Self {
        Color { value: "dark_aqua".to_string() }
    }

    pub fn dark_red() -> Self {
        Color { value: "dark_red".to_string() }
    }

    pub fn dark_purple() -> Self {
        Color { value: "dark_purple".to_string() }
    }

    pub fn gold() -> Self {
        Color { value: "gold".to_string() }
    }

    pub fn gray() -> Self {
        Color { value: "gray".to_string() }
    }

    pub fn dark_gray() -> Self {
        Color { value: "dark_gray".to_string() }
    }

    pub fn blue() -> Self {
        Color { value: "blue".to_string() }
    }

    pub fn green() -> Self {
        Color { value: "green".to_string() }
    }

    pub fn aqua() -> Self {
        Color { value: "aqua".to_string() }
    }

    pub fn red() -> Self {
        Color { value: "red".to_string() }
    }

    pub fn light_purple() -> Self {
        Color { value: "light_purple".to_string() }
    }

    pub fn yellow() -> Self {
        Color { value: "yellow".to_string() }
    }

    pub fn white() -> Self {
        Color { value: "white".to_string() }
    }

    pub fn reset() -> Self {
        Color::white()
    }

    pub fn from_hex(hex: String) -> Self {
        Color { value: hex }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { value: format!("#{:02X}{:02X}{:02X}", r, g, b) }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(i32),
    CopyToClipboard(String)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HoverEvent {
    ShowText(String),
    ShowItem(HoverItemDisplay),
    ShowEntity(String)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HoverItemDisplay {
    pub id: NamespaceId,
    pub count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HoverEntityDisplay {
    #[serde(rename = "type")]
    _type: NamespaceId,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>
}