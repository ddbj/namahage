use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::data::Data;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UnsortedPosition {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
}

impl Base for UnsortedPosition {
    fn id() -> &'static str {
        "JV_VR0026"
    }

    fn name() -> &'static str {
        "Data/UnsortedPosition"
    }
}

impl Default for UnsortedPosition {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Warning,
            message: match Config::language() {
                Lang::EN => String::from("Positions must be sorted numerically, in increasing order, within each reference sequence CHROM."),
                Lang::JA => String::from("POSは各参照配列CHROMの中では昇順で数値ソートされている必要があります。"),
            },
        }
    }
}

impl UnsortedPosition {
    pub fn validate(&self, item: &Data) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        if item.previous_record.is_none() {
            return None;
        }

        if let (Some(prev), Some(curr)) = (&item.previous_record, &item.current_record) {
            // chromosome differs
            if prev.get(0) != curr.get(0) {
                return None;
            }

            if let (Some(str1), Some(str2)) = (prev.get(1), curr.get(1)) {
                if let (Ok(pos1), Ok(pos2)) = (i64::from_str(str1), i64::from_str(str2)) {
                    if pos1 <= pos2 {
                        return None;
                    }
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
                "NC_000002.11".to_owned(),
                "10007".to_owned(),
                "rs1572047073".to_owned(),
                "C".to_owned(),
                "A".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: Some(vec![
                "NC_000001.10".to_owned(),
                "10026".to_owned(),
                "rs1570391712".to_owned(),
                "A".to_owned(),
                "C".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            errors: BTreeMap::new(),
        };

        let v = UnsortedPosition::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_valid_first_record() {
        let item = Data {
            config: &Config::default(),
            faidx: None,
            validated: false,
            content: None,
            chromosomes: HashSet::new(),
            current_record: Some(vec![
                "NC_000001.10".to_owned(),
                "10002".to_owned(),
                "rs1570391692".to_owned(),
                "A".to_owned(),
                "C".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: BTreeMap::new(),
        };

        let v = UnsortedPosition::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_unsorted_position() {
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
            previous_record: Some(vec![
                "NC_000001.10".to_owned(),
                "10002".to_owned(),
                "rs1570391692".to_owned(),
                "A".to_owned(),
                "C".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            errors: BTreeMap::new(),
        };

        let v = UnsortedPosition::default().validate(&item);

        assert!(v.is_some());
    }
}
