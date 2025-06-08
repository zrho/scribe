use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Header {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub math: MathHeader,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MathHeader {
    #[serde(default)]
    pub macros: HashMap<String, String>,
}
