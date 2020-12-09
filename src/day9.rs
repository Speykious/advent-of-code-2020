use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day8.dat")?;
  let mut lines = Vec::new();
  for line in BufReader::new(file).lines() {
    lines.push(line?);
  }

  

  Ok(())
}