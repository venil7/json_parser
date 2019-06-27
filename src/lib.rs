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
  CurlyOpen,
  CurlyClose,
  SquareOpen,
  SquareClose,
  Coma,
  Colon,
  StringValue(String),
  NumberValue(f64),
  BoolValue(bool),
  NullValue,
}

// trait Tokenizer {
//   type Item;
//   fn take_until(&mut self, token: Self::Item) -> Result<Vec<Self::Item>>;
//   fn take_while(&mut self, token: Self::Item) -> Result<Vec<Self::Item>>;
//   fn skip(&mut self, token: Self::Item) -> Result<()>;
// }

pub fn peekable_str(s: &str) -> PeekableIter<char> {
  let v: Vec<char> = s.chars().collect();
  v.into_iter().peekable()
}

fn take_until(cs: &mut PeekableIter<char>, predicate: fn(char) -> bool) -> Result<Vec<char>> {
  let mut res: Vec<char> = vec![];
  while let Some(c) = cs.next() {
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

fn take_while(cs: &mut PeekableIter<char>, predicate: fn(char) -> bool) -> Result<Vec<char>> {
  let mut res = vec![];
  while let Some(&c) = cs.peek() {
    if predicate(c) {
      cs.next();
      res.push(c);
    } else {
      return Ok(res);
    }
  }
  Ok(res)
}

fn skip(cs: &mut PeekableIter<char>, ch: char) -> Result<()> {
  match cs.next() {
    Some(c) if c == ch => Ok(()),
    _ => Err(Error::Tokenize(format!("expected token `{}`", ch).into())),
  }
}

fn string_token(cs: &mut PeekableIter<char>) -> Result<Token> {
  skip(cs, '"')?;
  let chars = take_until(cs, |c| c == '"')?;
  Ok(Token::StringValue(chars.iter().collect()))
}

fn number_token(cs: &mut PeekableIter<char>) -> Result<Token> {
  let chars = take_while(cs, |c| {
    Regex::new(r"^\d$").unwrap().is_match(&c.to_string())
  })?;
  let num_string: String = chars.iter().collect();
  match num_string.parse() {
    Ok(num) => Ok(Token::NumberValue(num)),
    Err(pfe) => Err(Error::Tokenize(pfe.to_string())),
  }
}

fn keyword_token(cs: &mut PeekableIter<char>) -> Result<Token> {
  let chars = take_while(cs, |c| {
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

pub fn tokenize(cs: &mut PeekableIter<char>) -> Result<Vec<Token>> {
  let mut v: Vec<Token> = vec![];
  while let Some(c) = cs.peek() {
    match c {
      ' ' => {
        cs.next();
      }
      '{' => {
        v.push(Token::CurlyOpen);
        cs.next();
      }
      '}' => {
        v.push(Token::CurlyClose);
        cs.next();
      }
      '[' => {
        v.push(Token::SquareOpen);
        cs.next();
      }
      ']' => {
        v.push(Token::SquareClose);
        cs.next();
      }
      ',' => {
        v.push(Token::Coma);
        cs.next();
      }
      ':' => {
        v.push(Token::Colon);
        cs.next();
      }
      '"' => v.push(string_token(cs)?),
      '0'...'9' => v.push(number_token(cs)?),
      _ => v.push(keyword_token(cs)?),
    }
  }
  Ok(v)
}

#[test]
fn test_string_token() {
  let mut ps = peekable_str(r#""hello""#);
  let result = string_token(&mut ps);
  assert_eq!(result.unwrap(), Token::StringValue("hello".into()));
}

#[test]
fn test_number_token() {
  let mut ps = peekable_str(r#"123"#);
  let result = number_token(&mut ps);
  assert_eq!(result.unwrap(), Token::NumberValue(123.0));
}

#[test]
fn test_true_token() {
  let mut ps = peekable_str(r#"true"#);
  let result = keyword_token(&mut ps);
  assert_eq!(result.unwrap(), Token::BoolValue(true));
}

#[test]
fn test_false_token() {
  let mut ps = peekable_str(r#"false"#);
  let result = keyword_token(&mut ps);
  assert_eq!(result.unwrap(), Token::BoolValue(false));
}

#[test]
fn test_null_token() {
  let mut ps = peekable_str(r#"null"#);
  let result = keyword_token(&mut ps);
  assert_eq!(result.unwrap(), Token::NullValue);
}

#[test]
fn test_tokenize_token() {
  let mut ps = peekable_str(r#"{"str": "hello", "num": 123, "array":[true, false, null]}"#);
  let result = tokenize(&mut ps);
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
