use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::global::Global;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlankLine {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
}

impl Base for BlankLine {
    fn id() -> &'static str {
        "JV_VR0006"
    }

    fn name() -> &'static str {
        "Global/BlankLine"
    }
}

impl Default for BlankLine {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Error,
            message: match Config::language() {
                Lang::EN => String::from("Blank line found. This line will be ignored."),
                Lang::JA => {
                    String::from("VCFに空のデータ行が見つかりました、この行は無視されます。")
                }
            },
        }
    }
}

impl BlankLine {
    pub fn validate(&self, item: &Global) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        if let Some(content) = &item.current_content {
            println!("{:?}", content);
            if !content.1.is_empty() {
                return None;
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
    use std::collections::HashMap;

    use crate::vcf::Content;

    use super::*;

    #[test]
    fn test_valid() {
        let item = Global {
            config: &Config::default(),
            validated: false,
            header: true,
            count: 0,
            current_content: Some(Content(
                4,
                "NC_000001.10\t10001\trs1570391677\tT\tA\t.\t.\t.".to_owned(),
            )),
            previous_content: Some(Content(
                3,
                "#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO".to_owned(),
            )),
            errors: HashMap::new(),
        };

        let v = BlankLine::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_data_before_header() {
        let item = Global {
            config: &Config::default(),
            validated: false,
            header: true,
            count: 0,
            current_content: Some(Content(4, "".to_owned())),
            previous_content: Some(Content(
                3,
                "#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO".to_owned(),
            )),
            errors: HashMap::new(),
        };

        let v = BlankLine::default().validate(&item);

        assert!(v.is_some());
    }
}
