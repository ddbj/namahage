use std::path::PathBuf;

use structopt::clap::crate_description;
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};

use namahage::config::{Config, Lang};
use namahage::errors::{Error, Result};
use namahage::vcf::Reader;

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum ReportType {
    JSON,
    Markdown,
    TSV,
}

#[derive(StructOpt)]
#[structopt(about = crate_description!())]
pub struct Options {
    /// Output format
    #[structopt(long)]
    generate_config: bool,

    /// Output format
    #[structopt(short, long, possible_values = ReportType::VARIANTS, default_value = "tsv")]
    report_type: ReportType,

    /// Path to configuration file
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,

    /// Path to fasta file
    #[structopt(short, long, parse(from_os_str))]
    fasta: Option<PathBuf>,

    /// File to process
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    Config::init_language(Lang::JA)
        .map_err(|lang| Error::CLIError(format!("Failed to set language to {}", lang)))?;

    let opts: Options = Options::from_args();

    if opts.generate_config {
        println!("{}", serde_yaml::to_string(&Config::default())?);
        return Ok(());
    }

    let config = match &opts.config {
        Some(path) => Config::from_path(path)?,
        None => Config::default(),
    };

    if let Some(path) = &opts.input {
        let mut reader = Reader::from_path(path)?;

        if let Some(faidx) = opts.fasta {
            reader.set_faidx(faidx)?;
        }

        let report = reader.validate(&config);

        println!("{:#?}", report);
    }

    Ok(())
}
