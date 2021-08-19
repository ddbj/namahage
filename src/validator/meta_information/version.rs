use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::meta_information::MetaInformation;
use crate::validator::{Level, ValidationError};
use crate::vcf::Content;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Version {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
    pub allowed: Vec<String>,
}

impl Base for Version {
    fn id() -> &'static str {
        "JV_VR0012"
    }

    fn name() -> &'static str {
        "MetaInformation/Version"
    }
}

impl Default for Version {
    fn default() -> Version {
        Version {
            enabled: true,
            level: Level::Warning,
            message: match Config::language() {
                Lang::EN => {
                    String::from("Unexpected VCF version. Expected values are {{allowed}}.")
                }
                Lang::JA => String::from(
                    "予期しないVCFバージョンです。期待されるバージョンは{{allowed}}です。",
                ),
            },
            allowed: vec![String::from("VCFv4.2"), String::from("VCFv4.3")],
        }
    }
}

impl Version {
    pub fn validate(&self, item: &MetaInformation) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        let pattern = Regex::new(r"fileformat=").unwrap();

        let file_format: Vec<&Content> = item
            .contents
            .iter()
            .filter(|&x| pattern.is_match(x.1.as_str()))
            .collect();

        // A single `fileformat` line is always required,
        // must be the first line in the file.
        if file_format.len() == 1 {
            if let Some(&content) = file_format.get(0) {
                if content.0 == 1 {
                    if self
                        .allowed
                        .iter()
                        .any(|a| content.1 == format!("##fileformat={}", a))
                    {
                        return None;
                    }
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
    use crate::vcf::Content;

    use super::*;

    #[test]
    fn test_valid() {
        let item = MetaInformation {
            config: &Config::default(),
            validated: false,
            contents: vec![
                Content(1, String::from("##fileformat=VCFv4.3")),
                Content(2, String::from("##reference=GRCh37.p13")),
            ],
            errors: vec![],
        };

        let v = Version::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_disallowed_value() {
        let item = MetaInformation {
            config: &Config::default(),
            validated: false,
            contents: vec![
                Content(1, String::from("##fileformat=VCFv4.1")),
                Content(2, String::from("##reference=GRCh37.p13")),
            ],
            errors: vec![],
        };

        let v = Version::default().validate(&item);

        assert!(v.is_some());
    }
}
