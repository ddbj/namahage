use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::header::Header;
use crate::validator::{Level, Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DuplicatedHeader {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
}

impl Base for DuplicatedHeader {
    fn id() -> &'static str {
        "JV_VR0005"
    }

    fn name() -> &'static str {
        "Header/DuplicatedHeader"
    }
}

impl Default for DuplicatedHeader {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Warning,
            message: match Config::language() {
                Lang::EN => String::from("Multiple header lines starting with # were found. All but the first header will be ignored."),
                Lang::JA => String::from(
                    "#から始まるヘッダー行が複数見つかりました。最初のヘッダー以外は無視されます。",
                ),
            },
        }
    }
}

impl Validate for DuplicatedHeader {
    type Item = Header;

    fn validate(&self, item: &Self::Item) -> Option<ValidationError> {
        if item.contents.len() == 1 {
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

        let v = DuplicatedHeader::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_duplicated_header() {
        let item = Header {
            contents: vec![
                Content(
                    2,
                    String::from("#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO"),
                ),
                Content(
                    3,
                    String::from("#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO"),
                ),
            ],
            errors: vec![],
        };

        let v = DuplicatedHeader::default().validate(&item);

        assert!(v.is_some());
    }
}
