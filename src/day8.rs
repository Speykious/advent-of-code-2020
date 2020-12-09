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

  let instructions: Vec<Inst> = lines.iter()
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
  
  let mut i: i32 = 0;
  let mut acc: i32 = 0;
  let mut executed: HashSet<i32> = HashSet::new();

  while executed.insert(i) {
    print!("(i={} acc={}) => ", i, acc);
    match instructions[i as usize] {
      Inst::Nop => { i += 1; print!("nop"); },
      Inst::Acc(n) => { acc += n; i += 1; print!("acc"); },
      Inst::Jmp(n) => { i += n; print!("jmp"); },
    }
    println!(" => (i={}, acc={})", i, acc);
  }

  println!("\nFinal value of the accumulator: {}", acc);
  Ok(())
}