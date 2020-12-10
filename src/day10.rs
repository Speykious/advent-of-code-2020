use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day10.dat")?;
  let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

  

  Ok(())
} 