use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::global::Global;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EmptyVCF {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
}

impl Base for EmptyVCF {
    fn id() -> &'static str {
        "JV_VR0007"
    }

    fn name() -> &'static str {
        "Global/EmptyVCF"
    }
}

impl Default for EmptyVCF {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Error,
            message: match Config::language() {
                Lang::EN => String::from("No records found in VCF."),
                Lang::JA => String::from("VCFにレコードが存在しません。"),
            },
        }
    }
}

impl EmptyVCF {
    pub fn validate(&self, item: &Global) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        if item.count > 0 {
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
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_valid() {
        let item = Global {
            config: &Config::default(),
            validated: false,
            header: true,
            count: 1,
            current_content: None,
            previous_content: None,
            errors: HashMap::new(),
        };

        let v = EmptyVCF::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_data_before_header() {
        let item = Global {
            config: &Config::default(),
            validated: false,
            header: true,
            count: 0,
            current_content: None,
            previous_content: None,
            errors: HashMap::new(),
        };

        let v = EmptyVCF::default().validate(&item);

        assert!(v.is_some());
    }
}
