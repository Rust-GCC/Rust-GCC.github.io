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
    let reports_org_path = reports::clone(cli.reports)?;
    let reports_org_list = reports::find(reports_org_path)?;
    for report in reports_org_list {
        let report_md = reports::convert(report, &cli.website)?;
        // add_report_to_website(new_md_report)?;
    }
    Ok(())
}
