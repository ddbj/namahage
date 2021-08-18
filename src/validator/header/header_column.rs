use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::header::Header;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HeaderColumn {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
}

impl Base for HeaderColumn {
    fn id() -> &'static str {
        "JV_VR0003"
    }

    fn name() -> &'static str {
        "Header/HeaderColumn"
    }
}

impl Default for HeaderColumn {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Error,
            message: match Config::language() {
                Lang::EN => String::from("The header line names the 8 fixed, mandatory columns. These columns are as follows: {{columns}}."),
                Lang::JA => {
                    String::from("ヘッダー行には8つの固定カラム{{columns}}が必須です。")
                }
            },
        }
    }
}

impl HeaderColumn {
    pub fn validate(&self, item: &Header) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        let expected = vec!["CHROM", "POS", "ID", "REF", "ALT", "QUAL", "FILTER", "INFO"];

        if item.contents.len() == 1 {
            if let Some(content) = item.contents.get(0) {
                if let Some(str) = content.1.strip_prefix("#") {
                    let mut columns = str.split("\t").collect::<Vec<&str>>();
                    columns.truncate(8);

                    if columns == expected {
                        return None;
                    }
                }
            }
        }

        let mut context = tera::Context::new();
        context.insert("columns", &expected.join(", "));

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
            config: &Config::default(),
            validated: false,
            contents: vec![Content(
                2,
                String::from("#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO"),
            )],
            errors: vec![],
        };

        let v = HeaderColumn::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_missing_column() {
        let item = Header {
            config: &Config::default(),
            validated: false,
            contents: vec![Content(
                2,
                String::from("#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER"),
            )],
            errors: vec![],
        };

        let v = HeaderColumn::default().validate(&item);

        assert!(v.is_some());
    }

    #[test]
    fn test_invalid_invalid_order() {
        let item = Header {
            config: &Config::default(),
            validated: false,
            contents: vec![Content(
                2,
                String::from("#CHROM\tPOS\tID\tREF\tALT\tQUAL\tINFO\tFILTER"),
            )],
            errors: vec![],
        };

        let v = HeaderColumn::default().validate(&item);

        assert!(v.is_some());
    }
}
