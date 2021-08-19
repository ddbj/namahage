use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::data::Data;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MultipleAlternateAlleles {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
}

impl Base for MultipleAlternateAlleles {
    fn id() -> &'static str {
        "JV_VR0038"
    }

    fn name() -> &'static str {
        "Data/MultipleAlternateAlleles"
    }
}

impl Default for MultipleAlternateAlleles {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Warning,
            message: match Config::language() {
                Lang::EN => String::from("The alternate sequence contains multiple variants."),
                Lang::JA => String::from("ALTに複数の変異が含まれます。"),
            },
        }
    }
}

impl MultipleAlternateAlleles {
    pub fn validate(&self, item: &Data) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        if let Some(record) = &item.current_record {
            if let Some(alternate) = record.get(4) {
                if !alternate.contains(",") {
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
        let item = Data {
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

        let v = MultipleAlternateAlleles::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_alt_is_multi_allelic() {
        let item = Data {
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
                "A,C".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: HashMap::default(),
        };

        let v = MultipleAlternateAlleles::default().validate(&item);

        assert!(v.is_some());
    }
}
