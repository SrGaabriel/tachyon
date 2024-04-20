use std::fmt::Display;

use lazy_static::lazy_static;
use regex::Regex;

pub struct NamespaceId {
    pub namespace: String,
    pub value: String
}

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^[a-z0-9_\-\.]+:[a-z0-9_\-/\.]+$").unwrap();
}

impl NamespaceId {
    pub fn new(namespace: String, value: String) -> Self {
        NamespaceId {
            namespace,
            value
        }
    }

    pub fn from_string(value: String) -> Self {
        let parts: Vec<&str> = value.split(':').collect();
        NamespaceId {
            namespace: parts[0].to_string(),
            value: parts[1].to_string()
        }
    }

    pub fn is_valid(&self) -> bool {
        IDENTIFIER_REGEX.is_match(&self.to_string())
    }
}

impl Display for NamespaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}:{}", self.namespace, self.value))
    }
}