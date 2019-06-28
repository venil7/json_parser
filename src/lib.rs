use regex::Regex;
use std::iter::{Iterator, Peekable};
use std::vec::IntoIter;

pub type PeekableIter<T> = Peekable<IntoIter<T>>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Tokenize(String),
}

#[derive(Debug, PartialEq)]
pub enum Token {
  Coma,
  Colon,
  CurlyOpen,
  CurlyClose,
  SquareOpen,
  SquareClose,
  StringValue(String),
  NumberValue(f64),
  BoolValue(bool),
  NullValue,
}

pub struct Tokenizer {
  char_stream: PeekableIter<char>,
}

impl Tokenizer {
  pub fn new(s: &str) -> Tokenizer {
    let vec: Vec<char> = s.chars().collect();
    let char_stream = vec.into_iter().peekable();
    Tokenizer { char_stream }
  }

  fn take_until(&mut self, predicate: fn(char) -> bool) -> Result<Vec<char>> {
    let mut res: Vec<char> = vec![];
    while let Some(c) = self.char_stream.next() {
      if !predicate(c) {
        res.push(c);
      } else {
        return Ok(res);
      }
    }
    let s: String = res.iter().collect();
    Err(Error::Tokenize(
      format!("unterminated token `{}`", s).into(),
    ))
  }

  fn take_while(&mut self, predicate: fn(char) -> bool) -> Result<Vec<char>> {
    let mut res = vec![];
    while let Some(&c) = self.char_stream.peek() {
      if predicate(c) {
        self.char_stream.next();
        res.push(c);
      } else {
        return Ok(res);
      }
    }
    Ok(res)
  }

  fn skip(&mut self, ch: char) -> Result<()> {
    match self.char_stream.next() {
      Some(c) if c == ch => Ok(()),
      _ => Err(Error::Tokenize(format!("expected token `{}`", ch).into())),
    }
  }

  fn string_token(&mut self) -> Result<Token> {
    self.skip('"')?;
    let chars = self.take_until(|c| c == '"')?;
    Ok(Token::StringValue(chars.iter().collect()))
  }

  fn number_token(&mut self) -> Result<Token> {
    let chars = self.take_while(|c| Regex::new(r"^\d$").unwrap().is_match(&c.to_string()))?;
    let num_string: String = chars.iter().collect();
    match num_string.parse() {
      Ok(num) => Ok(Token::NumberValue(num)),
      Err(pfe) => Err(Error::Tokenize(pfe.to_string())),
    }
  }

  fn keyword_token(&mut self) -> Result<Token> {
    let chars = self.take_while(|c| {
      Regex::new(r"^[a-zA-Z_\d]$")
        .unwrap()
        .is_match(&c.to_string())
    })?;
    let token: String = chars.iter().collect();
    match &token[..] {
      "true" => Ok(Token::BoolValue(true)),
      "false" => Ok(Token::BoolValue(false)),
      "null" => Ok(Token::NullValue),
      _ => Err(Error::Tokenize(
        format!("unrecognized token {}", token).into(),
      )),
    }
  }

  pub fn tokenize(&mut self) -> Result<Vec<Token>> {
    let mut v: Vec<Token> = vec![];
    while let Some(c) = self.char_stream.peek() {
      match c {
        ' ' => {
          self.char_stream.next();
        }
        '{' => {
          v.push(Token::CurlyOpen);
          self.char_stream.next();
        }
        '}' => {
          v.push(Token::CurlyClose);
          self.char_stream.next();
        }
        '[' => {
          v.push(Token::SquareOpen);
          self.char_stream.next();
        }
        ']' => {
          v.push(Token::SquareClose);
          self.char_stream.next();
        }
        ',' => {
          v.push(Token::Coma);
          self.char_stream.next();
        }
        ':' => {
          v.push(Token::Colon);
          self.char_stream.next();
        }
        '"' => v.push(self.string_token()?),
        '0'...'9' => v.push(self.number_token()?),
        _ => v.push(self.keyword_token()?),
      }
    }
    Ok(v)
  }
}

#[test]
fn test_string_token() {
  let mut tokenizer = Tokenizer::new(r#""hello""#);
  let result = tokenizer.string_token();
  assert_eq!(result.unwrap(), Token::StringValue("hello".into()));
}

#[test]
fn test_number_token() {
  let mut tokenizer = Tokenizer::new(r#"123"#);
  let result = tokenizer.number_token();
  assert_eq!(result.unwrap(), Token::NumberValue(123.0));
}

#[test]
fn test_true_token() {
  let mut tokenizer = Tokenizer::new(r#"true"#);
  let result = tokenizer.keyword_token();
  assert_eq!(result.unwrap(), Token::BoolValue(true));
}

#[test]
fn test_false_token() {
  let mut tokenizer = Tokenizer::new(r#"false"#);
  let result = tokenizer.keyword_token();
  assert_eq!(result.unwrap(), Token::BoolValue(false));
}

#[test]
fn test_null_token() {
  let mut tokenizer = Tokenizer::new(r#"null"#);
  let result = tokenizer.keyword_token();
  assert_eq!(result.unwrap(), Token::NullValue);
}

#[test]
fn test_tokenize_token() {
  let mut tokenizer =
    Tokenizer::new(r#"{"str": "hello", "num": 123, "array":[true, false, null]}"#);
  let result = tokenizer.tokenize();
  assert_eq!(
    result.unwrap(),
    [
      Token::CurlyOpen,
      Token::StringValue("str".into()),
      Token::Colon,
      Token::StringValue("hello".into()),
      Token::Coma,
      Token::StringValue("num".into()),
      Token::Colon,
      Token::NumberValue(123.0),
      Token::Coma,
      Token::StringValue("array".into()),
      Token::Colon,
      Token::SquareOpen,
      Token::BoolValue(true),
      Token::Coma,
      Token::BoolValue(false),
      Token::Coma,
      Token::NullValue,
      Token::SquareClose,
      Token::CurlyClose
    ]
  );
}
