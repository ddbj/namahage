use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::meta_information::MetaInformation;
use crate::validator::{Level, Validate, ValidationError};
use crate::vcf::Content;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileFormat {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
    pub allowed: Vec<String>,
}

impl Base for FileFormat {
    fn id() -> &'static str {
        "JV_VR0001"
    }

    fn name() -> &'static str {
        "MetaInformation/FileFormat"
    }
}

impl Default for FileFormat {
    fn default() -> FileFormat {
        FileFormat {
            enabled: true,
            level: Level::Error,
            message: match Config::language() {
                Lang::EN => String::from(
                    "A single `fileformat` line is always required, must be the first line in the file. Allowed values are {{allowed}}.",
                ),
                Lang::JA => String::from(
                    "ファイルの先頭に`fileformat`行が必ず1つ必要です。許可される値は{{allowed}}です。",
                ),
            },
            allowed: vec![String::from("VCFv4.2"), String::from("VCFv4.3")],
        }
    }
}

impl Validate for FileFormat {
    type Item = MetaInformation;

    fn validate(&self, item: &Self::Item) -> Option<ValidationError> {
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
        context.insert("allowed", &self.allowed.join("/"));

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
            contents: vec![
                Content(1, String::from("##fileformat=VCFv4.3")),
                Content(2, String::from("##reference=GRCh37.p13")),
            ],
            errors: vec![],
        };

        let v = FileFormat::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_not_first_line() {
        let item = MetaInformation {
            contents: vec![
                Content(2, String::from("##fileformat=VCFv4.3")),
                Content(3, String::from("##reference=GRCh37.p13")),
            ],
            errors: vec![],
        };

        let v = FileFormat::default().validate(&item);

        assert!(v.is_some());
    }

    #[test]
    fn test_invalid_disallowed_value() {
        let item = MetaInformation {
            contents: vec![
                Content(1, String::from("##fileformat=VCFv4.1")),
                Content(2, String::from("##reference=GRCh37.p13")),
            ],
            errors: vec![],
        };

        let v = FileFormat::default().validate(&item);

        assert!(v.is_some());
    }
}
