use crate::config::Config;
use crate::validator::{Validate, ValidationError};
use crate::vcf::Content;

pub mod duplicated_header;
pub mod header_column;
pub mod header_line;

#[derive(Debug)]
pub struct Header {
    contents: Vec<Content>,
    errors: Vec<ValidationError>,
}

impl Header {
    pub fn new() -> Header {
        Header {
            contents: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn push(&mut self, content: Content) {
        self.contents.push(content)
    }

    pub fn validate(&mut self, config: &Config) -> &Self {
        self.errors.clear();

        if config.duplicated_header.enabled {
            if let Some(e) = config.duplicated_header.validate(self) {
                self.errors.push(e);
            }
        }
        if config.header_column.enabled {
            if let Some(e) = config.header_column.validate(self) {
                self.errors.push(e);
            }
        }
        if config.header_line.enabled {
            if let Some(e) = config.header_line.validate(self) {
                self.errors.push(e);
            }
        }

        self
    }
}
