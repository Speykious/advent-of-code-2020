use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use std::fmt;

const C: &str = "\x1b[0m";
const B: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const GRN: &str = "\x1b[32m";
const YEL: &str = "\x1b[33m";
const BLU: &str = "\x1b[34m";
const MAG: &str = "\x1b[35m";

#[derive(Debug, Clone, Copy)]
enum Inst {
  /// The first value is the AND (0 override) mask, and the second is the OR (1 override) mask.
  Mask(u64, u64),
  /// The first value is the index of the memory cell, and the second is the value to make it be.
  Mem(u64, u64),
}

impl fmt::Display for Inst {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Inst::Mask(mand, mor)   =>
        write!(f, "{}Mask{} {}&& {:036b}{}\n     {}|| {:036b}{}",
               B, C, YEL, mand, C, RED, mor, C),
      Inst::Mem(index, value) =>
        write!(f, "{}Mem{}[{}{:08x}{}] = {}{:08x}{}",
               B, C, GRN, index, C, MAG, value, C),
    }
  }
}

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day14.dat")?;
  let data: Vec<_> = BufReader::new(file).lines()
    .map(|l| {
      let l = l.unwrap();
      let split = l.split(" = ").collect::<Vec<_>>();
      let (left, right) = (split[0].to_string(), split[1].to_string());
      if left == "mask" {
        right.chars().enumerate().fold(
          Inst::Mask((1 << 36) - 1, 0),
          |m, (i, c)| match m {
            Inst::Mask(mut mand, mut mor) => {
              let b = 1 << (35 - i as u64);
              match c {
                '1' => mor = mor | b,
                '0' => mand = mand & !b,
                _   => (),
              }
              Inst::Mask(mand, mor)
            },
            _ => panic!("Impossible...!"),
          }
        )
      } else {
        let index: u64 = left[4..left.len()-1].parse().unwrap();
        let value: u64 = right.parse().unwrap();
        Inst::Mem(index, value)
      }
    }).collect();
  
  for d in &data {
    println!("{}", d);
  }

  Ok(())
}
