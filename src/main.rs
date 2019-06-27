use json_parser::{Result, peekable_str, tokenize};

#[warn(dead_code)]
fn main() -> Result<()> {
  let mut ps = peekable_str(r#"{"field": "123"}"#);
  let ts = tokenize(&mut ps)?;
  println!("{:?}", ts);
  Ok(())
}