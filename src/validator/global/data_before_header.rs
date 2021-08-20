use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::global::Global;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataBeforeHeader {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
}

impl Base for DataBeforeHeader {
    fn id() -> &'static str {
        "JV_VR0004"
    }

    fn name() -> &'static str {
        "Global/DataBeforeHeader"
    }
}

impl Default for DataBeforeHeader {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Warning,
            message: match Config::language() {
                Lang::EN => {
                    String::from("Data line found before header line. This line will be ignored.")
                }
                Lang::JA => String::from(
                    "ヘッダー行より前にデータが記述されています。この行は無視されます。",
                ),
            },
        }
    }
}

impl DataBeforeHeader {
    pub fn validate(&self, item: &Global) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        if item.header {
            return None;
        }

        if let Some(content) = &item.current_content {
            if content.1.starts_with("#") {
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
    use std::collections::BTreeMap;

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
            errors: BTreeMap::new(),
        };

        let v = DataBeforeHeader::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_data_before_header() {
        let item = Global {
            config: &Config::default(),
            validated: false,
            header: false,
            count: 0,
            current_content: Some(Content(
                3,
                "NC_000001.10\t10001\trs1570391677\tT\tA\t.\t.\t.".to_owned(),
            )),
            previous_content: Some(Content(2, "##reference=GRCh37.p13".to_owned())),
            errors: BTreeMap::new(),
        };

        let v = DataBeforeHeader::default().validate(&item);

        assert!(v.is_some());
    }
}
