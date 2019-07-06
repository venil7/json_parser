use json_parser::parser::Json;
use json_parser::result::Result;
use std::env;
use std::fs;

#[warn(dead_code)]
fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    let json_string = fs::read_to_string(&args[1])?;
    let value: Json = json_string.parse::<Json>()?;
    println!("{:?}", value);
  }
  Ok(())
}
