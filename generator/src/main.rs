use std::path::PathBuf;

use clap::Parser;

mod error;
mod reports;

use error::Error;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, env = "WEBSITE")]
    website: PathBuf,
    #[arg(short, long, env = "REPORTS")]
    reports: String,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    println!("{}", reports::clone(cli.reports)?.display());
    Ok(())
}
