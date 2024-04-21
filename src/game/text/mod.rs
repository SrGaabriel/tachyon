use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use crate::game::text::style::Style;

pub(crate) mod serialization;

pub mod style;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextComponent {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<TextComponent>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    #[serde(rename = "type")]
    pub _type: String,
}

impl TextComponent {
    pub fn text(text: String) -> Self {
        TextComponent {
            text,
            extra: vec![],
            style: None,
            _type: "text".to_string()
        }
    }

    pub fn bold(mut self) -> Self {
        match self.style {
            Some(ref mut style) => style.bold = true,
            None => {
                self.style = Some(Style {
                    bold: true,
                    ..Default::default()
                });
            }
        }
        self
    }

    pub fn append(mut self, component: TextComponent) -> Self {
        self.extra.push(component);
        self
    }
}

#[macro_export]
macro_rules! text {
    ($text:expr) => {
        TextComponent::text($text.to_string())
    };
    ($text:expr, $($extra:expr),*) => {
        TextComponent::text($text.to_string())$(.append(TextComponent::text($extra.to_string())))*
    };
}
