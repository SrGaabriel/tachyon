use std::fmt::Display;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamespaceId {
    pub namespace: String,
    pub value: String
}

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^[a-z0-9_\-\.]+:[a-z0-9_\-/\.]+$").unwrap();
}

impl NamespaceId {
    pub fn create(namespace: String, value: String) -> Self {
        NamespaceId {
            namespace,
            value
        }
    }

    pub fn new(namespace: &str, value: &str) -> Self {
        NamespaceId {
            namespace: namespace.to_string(),
            value: value.to_string()
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

impl Serialize for NamespaceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'a> Deserialize<'a> for NamespaceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'a> {
        let value = String::deserialize(deserializer)?;
        Ok(NamespaceId::from_string(value))
    }
}