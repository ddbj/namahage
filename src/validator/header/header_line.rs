use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::header::Header;
use crate::validator::{Level, Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HeaderLine {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
}

impl Base for HeaderLine {
    fn id() -> &'static str {
        "JV_VR0002"
    }

    fn name() -> &'static str {
        "Header/HeaderLine"
    }
}

impl Default for HeaderLine {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Error,
            message: match Config::language() {
                Lang::EN => String::from(
                    "The header line is missing. The line starts with `#` is required.",
                ),
                Lang::JA => {
                    String::from("ヘッダー行が見つかりません。#から始まるヘッダー行が必要です。")
                }
            },
        }
    }
}

impl Validate for HeaderLine {
    type Item = Header;

    fn validate(&self, item: &Self::Item) -> Option<ValidationError> {
        if item.contents.len() > 0 {
            return None;
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
    use crate::vcf::Content;

    use super::*;

    #[test]
    fn test_valid() {
        let item = Header {
            contents: vec![Content(
                2,
                String::from("#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO"),
            )],
            errors: vec![],
        };

        let v = HeaderLine::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_missing_header_row() {
        let item = Header {
            contents: vec![],
            errors: vec![],
        };

        let v = HeaderLine::default().validate(&item);

        assert!(v.is_some());
    }
}
