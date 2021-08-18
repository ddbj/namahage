use std::collections::HashMap;

use crate::config::Config;
use crate::validator::ValidationError;
use crate::vcf::Content;

pub mod blank_line;
pub mod data_before_header;
pub mod empty_vcf;

#[derive(Debug)]
pub struct Global<'a> {
    config: &'a Config,
    validated: bool,
    header: bool,
    count: i64,
    current_content: Option<Content>,
    previous_content: Option<Content>,
    errors: HashMap<Option<Content>, Vec<ValidationError>>,
}

impl<'a> Global<'a> {
    pub fn new(config: &'a Config) -> Self {
        Global {
            config,
            validated: false,
            header: false,
            count: 0,
            current_content: None,
            previous_content: None,
            errors: HashMap::new(),
        }
    }

    fn push_or_insert(&mut self, key: Option<Content>, value: ValidationError) {
        let entry = self.errors.entry(key).or_insert(vec![]);
        entry.push(value)
    }

    pub fn push(&mut self, content: &Content) {
        if self.validated {
            return;
        }

        self.current_content = Some(content.to_owned());

        if let Some(e) = self.config.data_before_header.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }

        self.previous_content = self.current_content.to_owned();
    }

    pub fn validate(&mut self) -> &Self {
        if self.validated {
            return self;
        }

        if let Some(e) = self.config.empty_vcf.validate(self) {
            self.push_or_insert(None, e);
        }

        self.validated = true;

        self
    }
}
