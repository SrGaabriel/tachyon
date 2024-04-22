use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use crate::game::text::style::{ClickEvent, Color, HoverEvent, Style};

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

#[macro_export]
macro_rules! mutate_style {
    ($self:ident, $field:ident, $value:expr) => {
        match $self.style {
            Some(ref mut style) => style.$field = $value,
            None => {
                $self.style = Some(Style {
                    $field: $value,
                    ..Default::default()
                });
            }
        }
    };
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
        mutate_style!(self, bold, true);
        self
    }

    pub fn italic(mut self) -> Self {
        mutate_style!(self, italic, true);
        self
    }

    pub fn underlined(mut self) -> Self {
        mutate_style!(self, underlined, true);
        self
    }

    pub fn strikethrough(mut self) -> Self {
        mutate_style!(self, strikethrough, true);
        self
    }

    pub fn obfuscated(mut self) -> Self {
        mutate_style!(self, obfuscated, true);
        self
    }

    pub fn unstylized(mut self) -> Self {
        self.style = None;
        self
    }

    pub fn click_event(mut self, event: ClickEvent) -> Self {
        mutate_style!(self, click_event, Some(event));
        self
    }

    pub fn hover_event(mut self, event: HoverEvent) -> Self {
        mutate_style!(self, hover_event, Some(event));
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        mutate_style!(self, color, Some(color.value));
        self
    }

    pub fn append(mut self, component: TextComponent) -> Self {
        self.extra.push(component);
        self
    }
}