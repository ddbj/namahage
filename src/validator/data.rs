use std::collections::{BTreeMap, HashSet};

use regex::Regex;

use crate::config::Config;
use crate::validator::ValidationError;
use crate::vcf::Content;

pub mod allowed_alternate_base;
pub mod allowed_reference_base;
pub mod ambiguous_alternate_base;
pub mod ambiguous_reference_base;
pub mod deletion_length;
pub mod discontiguous_chromosome;
pub mod identical_bases;
pub mod insertion_length;
pub mod mismatch_reference_base;
pub mod missing_alternate_base;
pub mod missing_reference_base;
pub mod multiple_alternate_alleles;
pub mod position_format;
pub mod unsorted_position;

#[derive(Debug)]
pub struct Data<'a> {
    config: &'a Config,
    faidx: Option<&'a rust_htslib::faidx::Reader>,
    validated: bool,
    content: Option<Content>,
    chromosomes: HashSet<String>,
    current_record: Option<Vec<String>>,
    previous_record: Option<Vec<String>>,
    pub errors: BTreeMap<Option<Content>, Vec<ValidationError>>,
}

fn regex_nucleotide() -> Regex {
    Regex::new(r"^(?i)[ACGTURYSWKMBDHVN]*").unwrap()
}

impl<'a> Data<'a> {
    pub fn new(config: &'a Config, faidx: Option<&'a rust_htslib::faidx::Reader>) -> Self {
        Data {
            config,
            faidx,
            validated: false,
            content: None,
            chromosomes: HashSet::new(),
            current_record: None,
            previous_record: None,
            errors: BTreeMap::new(),
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

        if content.1.starts_with("#") {
            return;
        }

        self.content = Some(content.to_owned());
        self.current_record = Some(content.1.split("\t").map(|x| x.to_owned()).collect());

        if let Some(e) = self.config.allowed_alternate_base.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.allowed_reference_base.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.ambiguous_alternate_base.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.ambiguous_reference_base.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.deletion_length.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.discontiguous_chromosome.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.identical_bases.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.insertion_length.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.mismatch_reference_base.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.missing_alternate_base.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.missing_reference_base.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.multiple_alternate_alleles.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.position_format.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }
        if let Some(e) = self.config.unsorted_position.validate(self) {
            self.push_or_insert(Some(content.to_owned()), e);
        }

        // store CHROM value to set
        if let Some(current_record) = &self.current_record {
            if let Some(chrom) = current_record.get(0) {
                self.chromosomes.insert(chrom.to_owned());
            }
        }

        self.previous_record = self.current_record.to_owned();
    }

    pub fn validate(&mut self) -> &Self {
        if self.validated {
            return self;
        }

        self.validated = true;

        self
    }
}
