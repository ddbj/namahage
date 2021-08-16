use crate::config::Config;
use crate::validator::{Validate, ValidationError};
use crate::vcf::Content;

pub mod file_format;

#[derive(Debug)]
pub struct MetaInformation {
    contents: Vec<Content>,
    errors: Vec<ValidationError>,
}

impl MetaInformation {
    pub fn new() -> MetaInformation {
        MetaInformation {
            contents: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn push(&mut self, content: Content) {
        self.contents.push(content)
    }

    pub fn validate(&mut self, config: &Config) -> &Self {
        self.errors.clear();

        if config.file_format.enabled {
            if let Some(e) = config.file_format.validate(self) {
                self.errors.push(e);
            }
        }

        self
    }
}
