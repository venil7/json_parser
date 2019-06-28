use json_parser::{Result, Tokenizer};

#[warn(dead_code)]
fn main() -> Result<()> {
  let mut tokenizer = Tokenizer::new(r#"{"field": "123"}"#);
  let ts = tokenizer.tokenize()?;
  println!("{:?}", ts);
  Ok(())
}
