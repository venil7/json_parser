use json_parser::{Result, Tokenizer};

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
