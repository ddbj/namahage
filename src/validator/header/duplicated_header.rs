use serde::{Deserialize, Serialize};

use crate::config::{Base, Lang, Message, LANG};
use crate::validator::meta_information::MetaInformation;
use crate::validator::{Level, Validate, ValidationResult};

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

impl Message for DuplicatedHeader {
    fn message(lang: &Lang) -> &'static str {
        match lang {
            Lang::EN => "",
            Lang::JA => {
                "#から始まるヘッダー行が複数見つかりました。最初のヘッダー以外は無視されます。"
            }
        }
    }
}

impl Default for DuplicatedHeader {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Warning,
            message: String::from(Self::message(LANG.get_or_init(|| Lang::default()))),
        }
    }
}

impl Validate for DuplicatedHeader {
    type Item = MetaInformation;
    type Config = Self;

    fn validate(&self, _item: &Self::Item) -> ValidationResult {
        let valid = true;

        ValidationResult {
            id: Self::id(),
            name: Self::name(),
            level: self.level,
            valid,
            message: "".to_string(),
        }
    }
}
