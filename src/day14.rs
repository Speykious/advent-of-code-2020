use std::fs::File;
use std::io::{self, BufRead, BufReader};

enum Inst {
  Mask(u64, u64),
  Mem(u64, u64),
}

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day14.dat")?;
  let lines: Vec<_> = BufReader::new(file).lines()
                      .map(|l| {
                        let l = l.unwrap();
                        let split = l.split(" = ").collect::<Vec<_>>();
                        (split[0].to_string(), split[1].to_string())
                      }).collect();
  
  println!("{:#?}", lines);

  Ok(())
}
