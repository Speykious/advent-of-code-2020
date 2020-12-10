use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day10.dat")?;
  let mut numbers: Vec<u32> = BufReader::new(file).lines()
                              .map(|l| l.unwrap().parse().unwrap()).collect();
  numbers.sort();

  println!("{:?}", numbers);

  Ok(())
} 