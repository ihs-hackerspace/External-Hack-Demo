use toy_arms::external::error::TAExternalError;

#[derive(Debug)]
pub enum Error {
  TAExternalError(TAExternalError),
  PatternScanError,
}
  
impl From<TAExternalError> for Error {
  fn from(error: TAExternalError) -> Self {
    Error::TAExternalError(error)
  }
}

pub type Result<T> = std::result::Result<T, Error>;