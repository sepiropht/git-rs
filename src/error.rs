use std::io;
use std::fmt;

pub enum TgitError {
  IoError(std::io::Error),
  NoDirectory,
  InvalidCommit,
  InvalidIndex,
}

impl fmt::Display for TgitError {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      &TgitError::IoError(ref e) => e.fmt(formatter),
      &TgitError::NoDirectory => formatter.write_str("No directory found"),
      &TgitError::InvalidCommit => formatter.write_str("The commit is invalid"),
      &TgitError::InvalidIndex => formatter.write_str("the index is corrupt")
    }
  }
}

impl From<io::Error> for TgitError {
  fn from(err: io::Error) -> TgitError {
    TgitError::IoError(err)
  }
}