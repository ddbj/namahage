use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::vcf::Content;

pub mod data;
pub mod global;
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

#[derive(Debug)]
pub struct ValidationError {
    pub id: &'static str,
    pub name: &'static str,
    pub level: Level,
    pub message: String,
}

#[derive(Debug)]
pub struct ValidationReport {
    pub errors: Vec<Error>,
    pub global: BTreeMap<Option<Content>, Vec<ValidationError>>,
    pub meta_information: Vec<ValidationError>,
    pub header: Vec<ValidationError>,
    pub record: BTreeMap<Option<Content>, Vec<ValidationError>>,
}
