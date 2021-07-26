use serde::{Deserialize, Serialize};

use crate::config;

pub mod header;
pub mod meta_information;

/// Failure level for validators.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    /// Default
    None,
    /// Continue collecting validation even if a validation fails.
    Warning,
    /// Stop collecting validation if a validation fails.
    Error,
}

impl Default for Level {
    fn default() -> Self {
        Level::None
    }
}

#[derive(Debug, Default)]
pub struct ValidationResult {
    pub id: &'static str,
    pub name: &'static str,
    pub level: Level,
    pub valid: bool,
    pub message: String,
}

pub trait Validate
where
    Self::Config: config::Base + config::Message,
{
    type Item;
    type Config;

    fn validate(&self, item: &Self::Item) -> ValidationResult;
}
