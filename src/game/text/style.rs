use std::ops::Not;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::game::identifier::NamespaceId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Style {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
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