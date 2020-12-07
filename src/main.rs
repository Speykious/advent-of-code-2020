use std::io::Result;
mod day1;
mod day3;
mod day5;
mod day6;
mod day7;
#[macro_use]
extern crate nom;

fn main() -> Result<()> {
  println!("\x1b[1mDay 7\x1b[0m");
  day7::main()
}