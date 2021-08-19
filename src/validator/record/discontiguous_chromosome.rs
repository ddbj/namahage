use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::record::Record;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiscontiguousChromosome {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
}

impl Base for DiscontiguousChromosome {
    fn id() -> &'static str {
        "JV_VR0025"
    }

    fn name() -> &'static str {
        "Record/DiscontiguousChromosome"
    }
}

impl Default for DiscontiguousChromosome {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Warning,
            message: match Config::language() {
                Lang::EN => String::from("CHROM must form a contiguous block within the VCF file."),
                Lang::JA => String::from("CHROMはVCFの中で連続したブロックである必要があります。"),
            },
        }
    }
}

impl DiscontiguousChromosome {
    pub fn validate(&self, item: &Record) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        if item.previous_record.is_none() {
            return None;
        }

        if let (Some(prev), Some(curr)) = (&item.previous_record, &item.current_record) {
            if prev.get(0) == curr.get(0) {
                return None;
            } else if let Some(chr) = curr.get(0) {
                if !item.chromosomes.contains(chr.as_str()) {
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
                "10002".to_owned(),
                "rs1570391692".to_owned(),
                "A".to_owned(),
                "C".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: HashMap::new(),
        };

        let v = DiscontiguousChromosome::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_valid_2() {
        let mut chromosomes = HashSet::new();
        chromosomes.insert("NC_000001.10".to_owned());

        let item = Record {
            config: &Config::default(),
            faidx: None,
            validated: false,
            content: None,
            chromosomes,
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
                "10001".to_owned(),
                "rs1570391677".to_owned(),
                "T".to_owned(),
                "A".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            errors: HashMap::new(),
        };

        let v = DiscontiguousChromosome::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_data_before_header() {
        let mut chromosomes = HashSet::new();
        chromosomes.insert("NC_000001.10".to_owned());
        chromosomes.insert("NC_000002.11".to_owned());

        let item = Record {
            config: &Config::default(),
            faidx: None,
            validated: false,
            content: None,
            chromosomes,
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
                "NC_000002.11".to_owned(),
                "10007".to_owned(),
                "rs1572047073".to_owned(),
                "C".to_owned(),
                "A".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            errors: HashMap::new(),
        };

        let v = DiscontiguousChromosome::default().validate(&item);

        assert!(v.is_some());
    }
}
