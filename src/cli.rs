use std::path::PathBuf;

use structopt::clap::crate_description;
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum ReportType {
    JSON,
    Markdown,
    TSV,
}

#[derive(StructOpt, Debug)]
#[structopt(about = crate_description!())]
pub struct Options {
    /// Output format
    #[structopt(short, long, possible_values = ReportType::VARIANTS, case_insensitive = true, default_value = "tsv")]
    report_type: ReportType,

    /// File to process
    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,
}
