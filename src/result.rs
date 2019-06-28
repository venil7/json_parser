#[derive(Debug)]
pub enum Error {
  Tokenize(String),
}

pub type Result<T> = std::result::Result<T, Error>;
