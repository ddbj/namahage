use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rust_htslib;

use crate::config::Config;
use crate::errors::{Error, Result};
use crate::validator::data::Data;
use crate::validator::global::Global;
use crate::validator::header::Header;
use crate::validator::meta_information::MetaInformation;
use crate::validator::ValidationReport;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Reader<R> {
    reader: BufReader<R>,
    faidx: Option<rust_htslib::faidx::Reader>,
}

impl Reader<File> {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<File>> {
        if path.as_ref().exists() {
            Ok(Reader::new(File::open(path)?))
        } else {
            Err(Error::FileNotFoundError(
                path.as_ref().to_string_lossy().to_string(),
            ))?
        }
    }
}

const CAPACITY: usize = 10 * 1024;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Content(pub usize, pub String);

impl PartialOrd<Self> for Content {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for Content {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "L{}: {}", self.0, self.1)
    }
}

impl<R: io::Read> Reader<R> {
    pub fn from_reader(reader: R) -> Result<Reader<R>> {
        Ok(Reader::new(reader))
    }

    fn new(reader: R) -> Reader<R> {
        Reader {
            reader: BufReader::with_capacity(CAPACITY, reader),
            faidx: None,
        }
    }

    pub fn set_faidx<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.faidx = Some(rust_htslib::faidx::Reader::from_path(path)?);
        Ok(())
    }

    pub fn faidx(&self) -> Option<&rust_htslib::faidx::Reader> {
        self.faidx.as_ref()
    }

    pub fn validate<'a>(&'a mut self, config: &'a Config) -> ValidationReport {
        let mut i = 0;
        let mut buf = Vec::with_capacity(CAPACITY);

        let mut errors: Vec<Error> = Vec::new();
        let mut meta_information = MetaInformation::new(config);
        let mut header = Header::new(config);
        let mut global = Global::new(config);
        let mut record = Data::new(config, self.faidx.as_ref());

        while self
            .reader
            .read_until(b'\n', &mut buf)
            .expect("Failed to read bytes")
            != 0
        {
            i += 1;

            let str = match String::from_utf8(buf.clone()) {
                Ok(s) => s,
                Err(_) => {
                    errors.push(Error::VCFReadUtf8Error(Content(
                        i,
                        String::from_utf8_lossy(buf.as_slice()).to_string(),
                    )));
                    continue;
                }
            };

            let content = Content(i, str.trim_end().to_owned());

            global.push(&content);

            match &content.1 {
                str if str.starts_with("##") => meta_information.push(&content),
                str if str.starts_with("#") => header.push(&content),
                _ => {}
            }

            record.push(&content);

            buf.clear();
        }

        global.validate();
        meta_information.validate();
        header.validate();
        record.validate();

        ValidationReport {
            errors,
            global: global.errors,
            meta_information: meta_information.errors,
            header: header.errors,
            record: record.errors,
        }
    }
}
