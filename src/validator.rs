use serde::{Deserialize, Serialize};

use crate::errors::Error;

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
pub struct ValidationReport<'a> {
    pub errors: Vec<Error>,
    pub global: global::Global<'a>,
    pub meta_information: meta_information::MetaInformation<'a>,
    pub header: header::Header<'a>,
    pub record: data::Data<'a>,
}
