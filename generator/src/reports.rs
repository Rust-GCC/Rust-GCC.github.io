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

fn get_date(report_org_path: &Path) -> Result<String, Error> {
    let content = fs::read_to_string(report_org_path)?;
    for line in content.lines() {
        if line.starts_with("#+date:") {
            let date = line
                .split(':')
                .nth(1)
                .ok_or(Error::InvalidDatePrefix)?
                .trim()
                .to_string();
            return Ok(date);
        }
    }
    Err(Error::NoDateInOrgReport)
}

pub fn convert(report_org_path: PathBuf, website: &Path) -> Result<PathBuf, Error> {
    let mut report_md_filename = get_date(&report_org_path)?;
    report_md_filename.push_str("-");
    report_md_filename.push_str(report_org_path.file_name().unwrap().to_str().unwrap());
    let mut report_md_path = website
        .join("_posts")
        .join(PathBuf::from(report_md_filename));
    report_md_path.set_extension("md");
    Command::new("pandoc")
        .arg("--from=org")
        .arg("--to=gfm")
        .arg(report_org_path)
        .arg("-o")
        .arg(&report_md_path)
        .spawn()?;
    dbg!(&report_md_path);
    Ok(report_md_path)
}
