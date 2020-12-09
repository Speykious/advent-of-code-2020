use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

#[derive(Debug)]
enum Inst { Nop, Acc(i32), Jmp(i32) }

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day8.dat")?;
  let mut lines = Vec::new();
  for line in BufReader::new(file).lines() {
    lines.push(line?);
  }

  let data: Vec<Inst> = lines.iter()
  .map(|line| {
    let mut iter = line.split_whitespace();
    let inst = iter.next().unwrap();
    let n = iter.next().unwrap().parse().unwrap();
    match inst {
      "nop" => Inst::Nop,
      "acc" => Inst::Acc(n),
      "jmp" => Inst::Jmp(n),
      _ => panic!("Panik!! Data corrupted!"),
    }
  }).collect();
  
  

  println!("{:?}", data);
  Ok(())
}