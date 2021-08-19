use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::record::Record;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissingReferenceBase {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
    pub disallowed: Vec<String>,
}

impl Base for MissingReferenceBase {
    fn id() -> &'static str {
        "JV_VR0029"
    }

    fn name() -> &'static str {
        "Record/MissingReferenceBase"
    }
}

impl Default for MissingReferenceBase {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Warning,
            message: match Config::language() {
                Lang::EN => String::from(
                    "The reference sequence is missing. Refrain from using {{disallowed}}.",
                ),
                Lang::JA => {
                    String::from("REFに塩基が指定されていません。{{disallowed}}は使用できません。")
                }
            },
            disallowed: vec![".".to_owned(), "-".to_owned()],
        }
    }
}

impl MissingReferenceBase {
    pub fn validate(&self, item: &Record) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        if let Some(record) = &item.current_record {
            if let Some(reference) = record.get(3) {
                if !self.disallowed.iter().any(|str| reference.contains(str)) {
                    return None;
                }
            }
        }

        let mut context = tera::Context::new();
        context.insert("disallowed", &self.disallowed.join(", "));

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

        let v = MissingReferenceBase::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_ref_is_dot() {
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
                ".".to_owned(),
                "A".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: HashMap::default(),
        };

        let v = MissingReferenceBase::default().validate(&item);

        assert!(v.is_some());
    }
}
