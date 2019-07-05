use std::convert::From;

#[derive(Debug)]
pub enum Error {
  Tokenize(String),
  Parse(String),
  IO(std::io::Error),
  Other(String),
}

impl From<std::io::Error> for Error {
  fn from(io_err: std::io::Error) -> Self {
    Error::IO(io_err)
  }
}