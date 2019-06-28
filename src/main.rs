use json_parser::result::Result;
use json_parser::tokenizer::Tokenizer;

use std::env;
use std::fs;
#[warn(dead_code)]
fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    match fs::read_to_string(&args[1]) {
      Ok(json_string) => println!("{:?}", json_string.parse::<Tokenizer>()?.tokenize()?),
      _ => println!("failed to open file.."),
    }
  }
  Ok(())
}
