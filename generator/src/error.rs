#[derive(Debug)]
pub enum Error {
    CloneFail(git2::Error),
    IO(std::io::Error),
    InvalidDatePrefix,
    NoDateInOrgReport,
}

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Error {
        Error::CloneFail(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::IO(error)
    }
}
