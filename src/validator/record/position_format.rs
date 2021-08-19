use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::record::Record;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PositionFormat {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
}

impl Base for PositionFormat {
    fn id() -> &'static str {
        "JV_VR0024"
    }

    fn name() -> &'static str {
        "Record/PositionFormat"
    }
}

impl Default for PositionFormat {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Warning,
            message: match Config::language() {
                Lang::EN => String::from("POS should be a number."),
                Lang::JA => String::from("POSは数値でなければなりません。"),
            },
        }
    }
}

impl PositionFormat {
    pub fn validate(&self, item: &Record) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        if let Some(record) = &item.current_record {
            if let Some(str) = record.get(1) {
                if usize::from_str(str).is_ok() {
                    return None;
                }
            }
        }

        let context = tera::Context::new();

        Some(ValidationError {
            id: Self::id(),
            name: Self::name(),
            level: self.level,
            message: Config::template().render(Self::name(), &context).unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;

    #[test]
    fn test_valid() {
        let item = Record {
            config: &Config::default(),
            faidx: None,
            validated: false,
            content: None,
            chromosomes: HashSet::new(),
            current_record: Some(vec![
                "NC_000001.10".to_owned(),
                "10001".to_owned(),
                "rs1570391677".to_owned(),
                "T".to_owned(),
                "A".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: HashMap::default(),
        };

        let v = PositionFormat::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_data_before_header() {
        let item = Record {
            config: &Config::default(),
            faidx: None,
            validated: false,
            content: None,
            chromosomes: HashSet::new(),
            current_record: Some(vec![
                "NC_000001.10".to_owned(),
                "".to_owned(),
                "rs1570391677".to_owned(),
                "T".to_owned(),
                "A".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: HashMap::default(),
        };

        let v = PositionFormat::default().validate(&item);

        assert!(v.is_some());
    }
}
