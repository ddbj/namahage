use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::config::{Base, Config, Lang};
use crate::validator::data::Data;
use crate::validator::{Level, ValidationError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MismatchReferenceBase {
    pub enabled: bool,
    pub level: Level,
    pub message: String,
}

impl Base for MismatchReferenceBase {
    fn id() -> &'static str {
        "JV_VR0033"
    }

    fn name() -> &'static str {
        "Data/MismatchReferenceBase"
    }
}

impl Default for MismatchReferenceBase {
    fn default() -> Self {
        Self {
            enabled: true,
            level: Level::Warning,
            message: match Config::language() {
                Lang::EN => {
                    String::from("The REF bases do not match the bases in the reference sequences. VCF = \"{{vcf}}\", FASTA = \"{{fasta}}\"")
                }
                Lang::JA => String::from(
                    "VCFのREFの塩基が参照配列の塩基と一致しません。VCF = \"{{vcf}}\", FASTA = \"{{fasta}}\"",
                ),
            },
        }
    }
}

impl MismatchReferenceBase {
    pub fn validate(&self, item: &Data) -> Option<ValidationError> {
        if !self.enabled {
            return None;
        }

        if item.faidx.is_none() {
            return None;
        }

        let vcf = item
            .current_record
            .as_ref()
            .and_then(|r| r.get(3))
            .and_then(|x| Some(x.to_owned()))
            .unwrap_or("Failed to obtain REF".to_owned());

        let (chr, pos) = match &item.current_record {
            Some(r) => (r.get(0), r.get(1).and_then(|str| usize::from_str(str).ok())),
            None => (None, None),
        };

        if chr.is_none() || pos.is_none() {
            return None;
        }

        let pos = pos.unwrap();
        if !(pos > 0) {
            unreachable!("position should be greater than 0 by spec.")
        }
        let begin = pos - 1;
        let end = begin + vcf.len() - 1;

        let fasta = match item.faidx.unwrap().fetch_seq(chr.unwrap(), begin, end) {
            Ok(bytes) => std::str::from_utf8(bytes)
                .map(|x| x.to_owned())
                .unwrap_or("Failed to obtain sequence from FASTA".to_owned()),
            Err(_) => "Failed to obtain sequence from FASTA".to_owned(),
        };

        if vcf.to_lowercase() == fasta.to_lowercase() {
            return None;
        }

        let mut context = tera::Context::new();
        context.insert("vcf", &vcf);
        context.insert("fasta", &fasta);

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
    use std::collections::{HashMap, HashSet};

    use super::*;

    fn open_faidx() -> rust_htslib::faidx::Reader {
        rust_htslib::faidx::Reader::from_path("test/GRCh37.p13.chr1.head20000bp.fa").unwrap()
    }

    #[test]
    fn test_valid() {
        let faidx = open_faidx();
        let item = Data {
            config: &Config::default(),
            faidx: Some(&faidx),
            validated: false,
            content: None,
            chromosomes: HashSet::new(),
            current_record: Some(vec![
                "NC_000001.10".to_owned(),
                "10001".to_owned(),
                "rs1570391677".to_owned(),
                "T".to_owned(),
                "A".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: HashMap::default(),
        };

        let v = MismatchReferenceBase::default().validate(&item);

        assert!(v.is_none());

        let item = Data {
            config: &Config::default(),
            faidx: Some(&faidx),
            validated: false,
            content: None,
            chromosomes: HashSet::new(),
            current_record: Some(vec![
                "NC_000001.10".to_owned(),
                "10108".to_owned(),
                "rs1377973775".to_owned(),
                "CAACCCT".to_owned(),
                "C".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: HashMap::default(),
        };

        let v = MismatchReferenceBase::default().validate(&item);

        assert!(v.is_none());
    }

    #[test]
    fn test_invalid_reference_differs_from_fasta() {
        let faidx = open_faidx();
        let item = Data {
            config: &Config::default(),
            faidx: Some(&faidx),
            validated: false,
            content: None,
            chromosomes: HashSet::new(),
            current_record: Some(vec![
                "NC_000001.10".to_owned(),
                "10001".to_owned(),
                "rs1570391677".to_owned(),
                "G".to_owned(),
                "A".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
                ".".to_owned(),
            ]),
            previous_record: None,
            errors: HashMap::default(),
        };

        let v = MismatchReferenceBase::default().validate(&item);

        assert!(v.is_some());
    }
}
