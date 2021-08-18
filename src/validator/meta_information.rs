use crate::config::Config;
use crate::validator::ValidationError;
use crate::vcf::Content;

pub mod file_format;
pub mod version;

#[derive(Debug)]
pub struct MetaInformation<'a> {
    config: &'a Config,
    validated: bool,
    contents: Vec<Content>,
    errors: Vec<ValidationError>,
}

impl<'a> MetaInformation<'a> {
    pub fn new(config: &'a Config) -> Self {
        MetaInformation {
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

        if let Some(e) = self.config.file_format.validate(self) {
            self.errors.push(e);
        }

        self.validated = true;

        self
    }
}
