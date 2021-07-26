use serde::{Deserialize, Serialize};

use crate::config::{Base, Lang, Message, LANG};
use crate::validator::meta_information::MetaInformation;
use crate::validator::{Level, Validate, ValidationResult};

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

impl Message for FileFormat {
    fn message(lang: &Lang) -> &'static str {
        match lang {
            Lang::EN => "",
            Lang::JA => "ヘッダー行にfileformatが必要です。許可される値は{allowed}です。",
        }
    }
}

impl Default for FileFormat {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Error,
            message: String::from(Self::message(LANG.get_or_init(|| Lang::default()))),
            allowed: vec![String::from("VCFv4.2"), String::from("VCFv4.3")],
        }
    }
}

impl Validate for FileFormat {
    type Item = MetaInformation;
    type Config = Self;

    fn validate(&self, item: &Self::Item) -> ValidationResult {
        let valid = item.content.iter().any(|x| {
            self.allowed
                .iter()
                .any(|y| x.eq(format!("##fileformat={}", y).as_str()))
        });

        ValidationResult {
            id: Self::id(),
            name: Self::name(),
            level: self.level,
            valid,
            message: "".to_string(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     fn config_str() -> &'static str {
//         r#"---
// Enabled: true
// Level: error
// Message: "ヘッダー行にfileformatが必要です。許可される値は{allowed}です。"
// Allowed:
//   - VCFv4.2
//   - VCFv4.3"#
//     }
//
//     #[test]
//     fn test_valid() {
//         let v: FileFormat =
//             serde_yaml::from_str(config_str()).expect("Error deserializing configuration");
//
//         let meta = MetaInformation {
//             content: vec![String::from("##fileformat=VCFv4.3")],
//         };
//
//         let result = v.validate(&meta);
//
//         assert_eq!(result.valid, true);
//     }
//
//     #[test]
//     fn test_invalid() {
//         let v: FileFormat =
//             serde_yaml::from_str(config_str()).expect("Error deserializing configuration");
//
//         let meta = MetaInformation {
//             content: vec![String::from("##fileformat=VCFv4.1")],
//         };
//
//         let result = v.validate(&meta);
//
//         assert_eq!(result.valid, false);
//     }
// }
