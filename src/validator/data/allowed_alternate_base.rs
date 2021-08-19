use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::data::Data;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllowedAlternateBase {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
    pub allowed: Vec<String>,
}

impl Base for AllowedAlternateBase {
    fn id() -> &'static str {
        "JV_VR0036"
    }

    fn name() -> &'static str {
        "Data/AllowedAlternateBase"
    }
}

impl Default for AllowedAlternateBase {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Warning,
            message: match Config::language() {
                Lang::EN => String::from(
                    "The alternate sequence contains characters not allowed. Available characters are {{allowed}}.",
                ),
                Lang::JA => String::from(
                    "ALTに使用できない文字が含まれます。使用できる文字は{{allowed}}です。",
                ),
            },
            allowed: vec![
                "A".to_owned(),
                "C".to_owned(),
                "G".to_owned(),
                "T".to_owned(),
                "U".to_owned(),
            ],
        }
    }
}

impl AllowedAlternateBase {
    pub fn validate(&self, item: &Data) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        if let Some(record) = &item.current_record {
            if let Some(alternate) = record.get(4) {
                if alternate
                    .split(",")
                    .all(|alt| alt.chars().all(|c| self.allowed.contains(&c.to_string())))
                {
                    return None;
                }
            }
        }

        let mut context = tera::Context::new();
        context.insert("allowed", &self.allowed.join(", "));

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
                "ACGTU".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: HashMap::default(),
        };

        let v = AllowedAlternateBase::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_alt_contains_n() {
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
                "ACGTUN".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: HashMap::default(),
        };

        let v = AllowedAlternateBase::default().validate(&item);

        assert!(v.is_some());
    }
}
