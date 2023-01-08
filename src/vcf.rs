use bgzip::BGZFReader;
use rust_htslib;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::config::Config;
use crate::errors::{Error, Result};
use crate::validator::data::Data;
use crate::validator::global::Global;
use crate::validator::header::Header;
use crate::validator::meta_information::MetaInformation;
use crate::validator::ValidationReport;

struct UncompressedFile {
    reader: BufReader<File>,
}

struct BGZipFile {
    reader: BGZFReader<File>,
}

trait Validatable {
    fn validate<'a>(&'a mut self, config: &'a Config) -> ValidationReport;
}

impl Validatable for UncompressedFile {
    fn validate<'a>(&'a mut self, config: &'a Config) -> ValidationReport {
        let mut i = 0;
        let mut buf = Vec::with_capacity(CAPACITY);

        let mut errors: Vec<Error> = Vec::new();
        let mut meta_information = MetaInformation::new(config);
        let mut header = Header::new(config);
        let mut global = Global::new(config);
        let mut record = Data::new(config, None);

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

            if i % 100 == 0 {
                eprint!("\rprocessed: {}", i);
            }
        }
        eprintln!("\rprocessed: {}", i);

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

impl Validatable for BGZipFile {
    fn validate<'a>(&'a mut self, config: &'a Config) -> ValidationReport {
        let mut i = 0;
        let mut buf = String::new();

        let errors: Vec<Error> = Vec::new();
        let mut meta_information = MetaInformation::new(config);
        let mut header = Header::new(config);
        let mut global = Global::new(config);
        let mut record = Data::new(config, None);

        while self
            .reader
            .read_line(&mut buf)
            .expect("Failed to read bytes")
            != 0
        {
            i += 1;

            let content = Content(i, buf.trim_end().to_owned());

            global.push(&content);

            match &content.1 {
                str if str.starts_with("##") => meta_information.push(&content),
                str if str.starts_with("#") => header.push(&content),
                _ => {}
            }

            record.push(&content);

            buf.clear();

            if i % 100 == 0 {
                eprint!("\rprocessed: {}", i);
            }
        }
        eprintln!("\rprocessed: {}", i);

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

pub struct Reader {
    file: Box<dyn Validatable>,
    faidx: Option<rust_htslib::faidx::Reader>,
}

impl Reader {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().exists() {
            match path.as_ref().extension().and_then(|x| x.to_str()) {
                Some("bgz") | Some("gz") => Ok(Reader {
                    file: Box::new(BGZipFile {
                        reader: BGZFReader::new(File::open(path)?),
                    }),
                    faidx: None,
                }),
                Some("vcf") => Ok(Reader {
                    file: Box::new(UncompressedFile {
                        reader: BufReader::with_capacity(CAPACITY, File::open(path)?),
                    }),
                    faidx: None,
                }),
                _ => Err(Error::FileNotFoundError(
                    path.as_ref().to_string_lossy().to_string(),
                ))?,
            }
        } else {
            Err(Error::FileNotFoundError(
                path.as_ref().to_string_lossy().to_string(),
            ))?
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
        self.file.validate(config)
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
