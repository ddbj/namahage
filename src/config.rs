use std::fmt::{Display, Formatter};
use std::fs::File;
use std::path::Path;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::errors::Result;
use crate::validator::{header, meta_information};

#[derive(Debug, Copy, Clone)]
pub enum Lang {
    EN,
    JA,
}

impl Display for Lang {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Lang::EN => write!(f, "EN"),
            Lang::JA => write!(f, "JA"),
        }
    }
}

impl Default for Lang {
    fn default() -> Self {
        Lang::EN
    }
}

pub static LANG: OnceCell<Lang> = OnceCell::new();

pub trait Base {
    fn id() -> &'static str;
    fn name() -> &'static str;
}

pub trait Message {
    fn message(lang: &Lang) -> &'static str;
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "MetaInformation/FileFormat")]
    pub file_format: meta_information::file_format::FileFormat,

    #[serde(rename = "Header/HeaderLine")]
    pub header_line: header::header_line::HeaderLine,
    #[serde(rename = "Header/HeaderColumn")]
    pub header_column: header::header_column::HeaderColumn,
    #[serde(rename = "Header/DuplicatedHeader")]
    pub duplicated_header: header::duplicated_header::DuplicatedHeader,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config: Config = serde_yaml::from_reader(File::open(path)?)?;

        Ok(config)
    }
}
