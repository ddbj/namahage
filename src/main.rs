use anyhow::Result;
use structopt::StructOpt;

use namahage::cli::Options;

fn main() -> Result<()> {
    Options::from_args();

    Ok(())
}
