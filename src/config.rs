use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use tera::Tera;

use crate::errors::Result;
use crate::validator::{global, header, meta_information};

const CONFIG_MESSAGE_KEY: &'static str = "Message";

static LANG: OnceCell<Lang> = OnceCell::new();
static TERA: OnceCell<Tera> = OnceCell::new();

#[derive(Debug, Copy, Clone)]
pub enum Lang {
    EN,
    JA,
}

impl Default for Lang {
    fn default() -> Lang {
        Lang::EN
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Lang::EN => write!(f, "EN"),
            Lang::JA => write!(f, "JA"),
        }
    }
}

pub trait Base {
    fn id() -> &'static str;
    fn name() -> &'static str;
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "MetaInformation/FileFormat")]
    pub file_format: meta_information::file_format::FileFormat,
    #[serde(rename = "MetaInformation/Version")]
    pub version: meta_information::version::Version,

    #[serde(rename = "Header/HeaderLine")]
    pub header_line: header::header_line::HeaderLine,
    #[serde(rename = "Header/HeaderColumn")]
    pub header_column: header::header_column::HeaderColumn,
    #[serde(rename = "Header/DuplicatedHeader")]
    pub duplicated_header: header::duplicated_header::DuplicatedHeader,

    #[serde(rename = "Global/DataBeforeHeader")]
    pub data_before_header: global::data_before_header::DataBeforeHeader,
    #[serde(rename = "Global/BlankLine")]
    pub blank_line: global::blank_line::BlankLine,
    #[serde(rename = "Global/EmptyVCF")]
    pub empty_vcf: global::empty_vcf::EmptyVCF,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Config> {
        let str = fs::read_to_string(&path)?;
        let config: Config = serde_yaml::from_str(str.as_str())?;

        TERA.get_or_init(|| config.init_tera());

        Ok(config)
    }

    pub fn init_language(lang: Lang) -> Result<(), Lang> {
        LANG.set(lang)
    }

    pub fn language<'a>() -> &'a Lang {
        LANG.get_or_init(|| Lang::default())
    }

    pub fn template<'a>() -> &'a Tera {
        TERA.get_or_init(|| {
            let config = Config::default();
            config.init_tera()
        })
    }

    fn init_tera(&self) -> Tera {
        let mut tera = Tera::default();

        let message_key = Value::String(CONFIG_MESSAGE_KEY.to_owned());

        let value = serde_yaml::to_value(self).expect("Failed to convert configuration");

        if let Value::Mapping(ref configurations) = value {
            for (k, v) in configurations.iter() {
                if let Value::String(key) = k {
                    if let Value::Mapping(configuration) = v {
                        if let Some(v) = configuration.get(&message_key) {
                            if let Value::String(message) = v {
                                tera.add_raw_template(key, message)
                                    .expect("Failed to register message template");
                            }
                        }
                    }
                }
            }
        }

        tera
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let config = Config::default();
        let serialize = serde_yaml::to_string(&config);

        assert!(serialize.is_ok());
        eprintln!("{}", serialize.unwrap());
    }
}
