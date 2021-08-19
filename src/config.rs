use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use tera::Tera;

use crate::errors::Result;
use crate::validator::{data, global, header, meta_information};

const CONFIG_MESSAGE_KEY: &'static str = "Message";

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

    #[serde(rename = "Data/AllowedAlternateBase")]
    pub allowed_alternate_base: data::allowed_alternate_base::AllowedAlternateBase,
    #[serde(rename = "Data/AllowedReferenceBase")]
    pub allowed_reference_base: data::allowed_reference_base::AllowedReferenceBase,
    #[serde(rename = "Data/AmbiguousAlternateBase")]
    pub ambiguous_alternate_base: data::ambiguous_alternate_base::AmbiguousAlternateBase,
    #[serde(rename = "Data/AmbiguousReferenceBase")]
    pub ambiguous_reference_base: data::ambiguous_reference_base::AmbiguousReferenceBase,
    #[serde(rename = "Data/DeletionLength")]
    pub deletion_length: data::deletion_length::DeletionLength,
    #[serde(rename = "Data/DiscontiguousChromosome")]
    pub discontiguous_chromosome: data::discontiguous_chromosome::DiscontiguousChromosome,
    #[serde(rename = "Data/IdenticalBases")]
    pub identical_bases: data::identical_bases::IdenticalBases,
    #[serde(rename = "Data/InsertionLength")]
    pub insertion_length: data::insertion_length::InsertionLength,
    #[serde(rename = "Data/MismatchReferenceBase")]
    pub mismatch_reference_base: data::mismatch_reference_base::MismatchReferenceBase,
    #[serde(rename = "Data/MissingAlternateBase")]
    pub missing_alternate_base: data::missing_alternate_base::MissingAlternateBase,
    #[serde(rename = "Data/MissingReferenceBase")]
    pub missing_reference_base: data::missing_reference_base::MissingReferenceBase,
    #[serde(rename = "Data/MultipleAlternateAlleles")]
    pub multiple_alternate_alleles: data::multiple_alternate_alleles::MultipleAlternateAlleles,
    #[serde(rename = "Data/PositionFormat")]
    pub position_format: data::position_format::PositionFormat,
    #[serde(rename = "Data/UnsortedPosition")]
    pub unsorted_position: data::unsorted_position::UnsortedPosition,
}

static LANG: OnceCell<Lang> = OnceCell::new();
static TERA: OnceCell<Tera> = OnceCell::new();

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
