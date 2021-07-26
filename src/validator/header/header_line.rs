use serde::{Deserialize, Serialize};

use crate::config::{Base, Lang, Message, LANG};
use crate::validator::meta_information::MetaInformation;
use crate::validator::{Level, Validate, ValidationResult};

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

impl Message for HeaderLine {
    fn message(lang: &Lang) -> &'static str {
        match lang {
            Lang::EN => "",
            Lang::JA => "ヘッダー行が見つかりません。#から始まるヘッダー行が必要です。",
        }
    }
}

impl Default for HeaderLine {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Error,
            message: String::from(Self::message(LANG.get_or_init(|| Lang::default()))),
        }
    }
}

impl Validate for HeaderLine {
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
