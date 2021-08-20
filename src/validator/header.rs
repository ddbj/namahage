use crate::config::Config;
use crate::validator::ValidationError;
use crate::vcf::Content;

pub mod duplicated_header;
pub mod header_column;
pub mod header_line;

#[derive(Debug)]
pub struct Header<'a> {
    config: &'a Config,
    validated: bool,
    contents: Vec<Content>,
    pub errors: Vec<ValidationError>,
}

impl<'a> Header<'a> {
    pub fn new(config: &'a Config) -> Self {
        Header {
            config,
            validated: false,
            contents: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn push(&mut self, content: &Content) {
        if self.validated {
            return;
        }

        self.contents.push(content.to_owned())
    }

    pub fn validate(&mut self) -> &Self {
        if self.validated {
            return self;
        }

        if let Some(e) = self.config.duplicated_header.validate(self) {
            self.errors.push(e);
        }
        if let Some(e) = self.config.header_column.validate(self) {
            self.errors.push(e);
        }
        if let Some(e) = self.config.header_line.validate(self) {
            self.errors.push(e);
        }

        self.validated = true;

        self
    }
}
