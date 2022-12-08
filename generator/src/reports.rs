use std::path::PathBuf;

use git2::Repository;
use mktemp::Temp;

use crate::error::Error;

pub fn clone(reports_repo: String) -> Result<PathBuf, Error> {
    let path_to_repo = Temp::new_dir()?.to_path_buf();
    Repository::clone(&reports_repo, &path_to_repo)?;
    Ok(path_to_repo)
}
