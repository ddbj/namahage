use serde::{Deserialize, Serialize};

use crate::config::{Base, Lang, Message, LANG};
use crate::validator::meta_information::MetaInformation;
use crate::validator::{Level, Validate, ValidationResult};

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

impl Message for HeaderColumn {
    fn message(lang: &Lang) -> &'static str {
        match lang {
            Lang::EN => "",
            Lang::JA => "ヘッダー行に必要なカラムがありません。[CHROM, POS, ID, REF, ALT, QUAL, FILTER, INFO]は必須項目です。",
        }
    }
}

impl Default for HeaderColumn {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Error,
            message: String::from(Self::message(LANG.get_or_init(|| Lang::default()))),
        }
    }
}

impl Validate for HeaderColumn {
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
