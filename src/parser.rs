use crate::error::Error;
use crate::result::Result;
use crate::tokenizer::Tokenizer;
use crate::tokenizer::{PeekableIter, Token};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Json {
  Null,
  Bool(bool),
  Number(f64),
  String(String),
  Array(Vec<Box<Json>>),
  Object(HashMap<String, Box<Json>>),
}

impl FromStr for Json {
  type Err = Error;
  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    let mut token_stream: PeekableIter<Token> =
      s.parse::<Tokenizer>()?.tokenize()?.into_iter().peekable();
    parse_item(&mut token_stream)
  }
}

fn skip_token(token_stream: &mut PeekableIter<Token>, token: Token) -> Result<()> {
  match token_stream.next() {
    Some(ref tkn) if *tkn == token => Ok(()),
    _ => Err(Error::Parse("token expected".into())),
  }
}

fn parse_array(token_stream: &mut PeekableIter<Token>) -> Result<Json> {
  skip_token(token_stream, Token::SquareOpen)?;
  let mut v = vec![];
  loop {
    match token_stream.peek() {
      Some(Token::SquareClose) => break,
      Some(_) => {
        v.push(Box::new(parse_item(token_stream)?));
        if let Some(Token::SquareClose) = token_stream.peek() {
          continue;
        }
        skip_token(token_stream, Token::Coma)?;
      }
      _ => return Err(Error::Parse("unterminated array".into())),
    }
  }
  skip_token(token_stream, Token::SquareClose)?;
  Ok(Json::Array(v))
}

fn parse_object(token_stream: &mut PeekableIter<Token>) -> Result<Json> {
  skip_token(token_stream, Token::CurlyOpen)?;
  let mut obj: HashMap<String, Box<Json>> = HashMap::default();
  loop {
    match token_stream.peek() {
      Some(Token::CurlyClose) => break,
      Some(_) => {
        if let Json::String(key) = parse_string(token_stream)? {
          skip_token(token_stream, Token::Colon)?;
          let val = parse_item(token_stream)?;
          obj.insert(key, Box::new(val));
          if let Some(Token::CurlyClose) = token_stream.peek() {
            continue;
          }
          skip_token(token_stream, Token::Coma)?;
        } else {
          return Err(Error::Parse("object key expected".into()));
        }
      }
      _ => return Err(Error::Parse("unterminated object".into())),
    }
  }
  skip_token(token_stream, Token::CurlyClose)?;
  return Ok(Json::Object(obj));
}

pub fn parse_item(token_stream: &mut PeekableIter<Token>) -> Result<Json> {
  match token_stream.peek() {
    Some(Token::NullValue) => parse_null(token_stream),
    Some(Token::BoolValue(_)) => parse_bool(token_stream),
    Some(Token::NumberValue(_)) => parse_number(token_stream),
    Some(Token::StringValue(_)) => parse_string(token_stream),
    Some(Token::SquareOpen) => parse_array(token_stream),
    Some(Token::CurlyOpen) => parse_object(token_stream),
    _ => Err(Error::Parse("unexpected token".into())),
  }
}

pub fn parse_null(token_stream: &mut PeekableIter<Token>) -> Result<Json> {
  match token_stream.next() {
    Some(Token::NullValue) => Ok(Json::Null),
    _ => Err(Error::Parse("null expected".into())),
  }
}

pub fn parse_bool(token_stream: &mut PeekableIter<Token>) -> Result<Json> {
  match token_stream.next() {
    Some(Token::BoolValue(b)) => Ok(Json::Bool(b)),
    _ => Err(Error::Parse("bool expected".into())),
  }
}

pub fn parse_number(token_stream: &mut PeekableIter<Token>) -> Result<Json> {
  match token_stream.next() {
    Some(Token::NumberValue(n)) => Ok(Json::Number(n)),
    _ => Err(Error::Parse("number expected".into())),
  }
}

pub fn parse_string(token_stream: &mut PeekableIter<Token>) -> Result<Json> {
  match token_stream.next() {
    Some(Token::StringValue(s)) => Ok(Json::String(s)),
    _ => Err(Error::Parse("string expected".into())),
  }
}
