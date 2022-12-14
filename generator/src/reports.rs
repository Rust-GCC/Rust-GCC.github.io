use std::path::{Path, PathBuf};
use std::process::Command;

use std::fs;

use git2::Repository;
use mktemp::Temp;

use crate::error::Error;

pub fn clone(reports_repo: String) -> Result<PathBuf, Error> {
    let path_to_repo = Temp::new_dir()?.to_path_buf();
    Repository::clone(&reports_repo, &path_to_repo)?;
    Ok(path_to_repo)
}

pub fn find(reports_org_path: PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut results = Vec::new();
    for entry in fs::read_dir(reports_org_path)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(Some("org")) = path.extension().map(|osstr| osstr.to_str()) {
            results.push(path);
        }
    }
    Ok(results)
}

pub fn convert(report_org_path: PathBuf, website: &Path) -> Result<PathBuf, Error> {
    let mut report_md_path = website
        .join("reporting")
        .join(PathBuf::from(report_org_path.clone().file_name().unwrap()));
    dbg!(&report_md_path);
    report_md_path.set_extension("md");
    Command::new("pandoc")
        .arg("--from=org")
        .arg("--to=gfm")
        .arg(report_org_path)
        .arg("-o")
        .arg(&report_md_path)
        .spawn()?;
    Ok(report_md_path)
}
