use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::data::Data;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AmbiguousReferenceBase {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
    pub disallowed: Vec<String>,
}

impl Base for AmbiguousReferenceBase {
    fn id() -> &'static str {
        "JV_VR0028"
    }

    fn name() -> &'static str {
        "Data/AmbiguousReferenceBase"
    }
}

impl Default for AmbiguousReferenceBase {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Warning,
            message: match Config::language() {
                Lang::EN => String::from(
                    "The reference sequence contains IUPAC ambiguity codes. Refrain from using {{disallowed}}.",
                ),
                Lang::JA => String::from(
                    "REFに曖昧な塩基が含まれています。{{disallowed}}は使用できません。",
                ),
            },
            disallowed: vec![
                "R".to_owned(),
                "Y".to_owned(),
                "S".to_owned(),
                "W".to_owned(),
                "K".to_owned(),
                "M".to_owned(),
                "B".to_owned(),
                "D".to_owned(),
                "H".to_owned(),
                "V".to_owned(),
                "N".to_owned(),
            ],
        }
    }
}

impl AmbiguousReferenceBase {
    pub fn validate(&self, item: &Data) -> Option<ValidationError> {
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
    use std::collections::{BTreeMap, HashSet};

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
            errors: BTreeMap::default(),
        };

        let v = AmbiguousReferenceBase::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_ref_contains_n() {
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
                "N".to_owned(),
                "A".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: BTreeMap::default(),
        };

        let v = AmbiguousReferenceBase::default().validate(&item);

        assert!(v.is_some());
    }
}
