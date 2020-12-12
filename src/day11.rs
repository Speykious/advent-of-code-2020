use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day11.dat")?;
  let mut lines: Vec<usize> = BufReader::new(file).lines()
                              .map(|l| l.unwrap()).collect();
  println!("{:?}", lines);

  Ok(())
}
