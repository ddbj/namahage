use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::record::Record;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeletionLength {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
    pub max: usize,
}

impl Base for DeletionLength {
    fn id() -> &'static str {
        "JV_VR0037"
    }

    fn name() -> &'static str {
        "Record/DeletionLength"
    }
}

impl Default for DeletionLength {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Warning,
            message: match Config::language() {
                Lang::EN => String::from(
                    "The length of the deletion exceeds the allowed value. Maximum of length is {{max}}.",
                ),
                Lang::JA => {
                    String::from("欠損される塩基の長さが許容値を超えています。上限は{{max}}です。")
                }
            },
            max: 50,
        }
    }
}

impl DeletionLength {
    pub fn validate(&self, item: &Record) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        let re = super::regex_nucleotide();

        if let Some(record) = &item.current_record {
            if let (Some(reference), Some(alternate)) = (record.get(3), record.get(4)) {
                let ref_len = match re.captures(reference) {
                    Some(cap) => cap[0].len(),
                    None => 0,
                };
                let alt_len = match re.captures(alternate) {
                    Some(cap) => cap[0].len(),
                    None => 0,
                };

                if (ref_len as i32) - (alt_len as i32) < (self.max as i32) {
                    return None;
                }
            }
        }

        let mut context = tera::Context::new();
        context.insert("max", &self.max);

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
    fn test_valid_length_of_deletion_is_within_limit() {
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
                std::iter::repeat("T").take(50).collect::<String>(),
                "T".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: HashMap::default(),
        };

        let v = DeletionLength::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_length_of_deletion_is_over_limit() {
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
                std::iter::repeat("T").take(51).collect::<String>(),
                "T".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: HashMap::default(),
        };

        let v = DeletionLength::default().validate(&item);

        assert!(v.is_some());
    }
}
