use json_parser::result::Result;
use json_parser::tokenizer::Tokenizer;

use std::env;
use std::fs;
#[warn(dead_code)]
fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  let json_string = fs::read_to_string(&args[1])?;
  let tokens = json_string.parse::<Tokenizer>()?.tokenize()?;
  if args.len() > 1 {
    println!("{:?}", tokens)
  }
  Ok(())
}
