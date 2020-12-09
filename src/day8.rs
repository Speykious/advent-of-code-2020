use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Inst { Nop(i32), Acc(i32), Jmp(i32) }

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day8.dat")?;
  let mut lines = Vec::new();
  for line in BufReader::new(file).lines() {
    lines.push(line?);
  }

  let mut instructions: Vec<Inst> = lines.iter()
  .map(|line| {
    let mut iter = line.split_whitespace();
    let inst = iter.next().unwrap();
    let n = iter.next().unwrap().parse().unwrap();
    match inst {
      "nop" => Inst::Nop(n),
      "acc" => Inst::Acc(n),
      "jmp" => Inst::Jmp(n),
      _ => panic!("Panik!! Data corrupted!"),
    }
  }).collect();
  
  let nop_jmps: Vec<usize> = instructions.iter().enumerate()
  .filter(|(_, inst)| match inst { Inst::Nop(_) | Inst::Jmp(_) => true, _ => false })
  .map(|(i, _)| i)
  .collect();

  for j in 0..nop_jmps.len() {
    // Had to come up with something clever to not borrow instructions as mutable and immutable
    { swap_nop_jmp(&mut instructions, nop_jmps[j]); }
    let (len, i, acc, terminates) = execute(&instructions, false);
    { swap_nop_jmp(&mut instructions, nop_jmps[j]); }
    println!("len={:<3} i={:<3} acc={:<4} | terminates={}", len, i, acc, terminates);
    if terminates { break }
  }
  
  Ok(())
}

fn swap_nop_jmp(instructions: &mut Vec<Inst>, idx: usize) {
  let nop_jmp = &mut instructions[idx];
  *nop_jmp = match nop_jmp {
    Inst::Nop(n) => Inst::Jmp(*n),
    Inst::Jmp(n) => Inst::Nop(*n),
    _ => panic!("Panik!! How am I getting an Inst::Acc?!"),
  };
}

fn execute(instructions: &Vec<Inst>, bprint: bool) -> (usize, i32, i32, bool) {
  let mut i: i32 = 0;
  let mut acc: i32 = 0;
  let mut executed: HashSet<i32> = HashSet::new();
  let mut terminates: bool = false;
  let len: i32 = instructions.len() as i32;
  
  while executed.insert(i) {
    if bprint { print!("i={:<3} acc={:<4}    => ", i, acc); }
    match instructions[i as usize] {
      Inst::Nop(_) => { i += 1; if bprint { print!("nop"); } },
      Inst::Acc(n) => { acc += n; i += 1; if bprint { print!("acc"); } },
      Inst::Jmp(n) => { i += n; if bprint { print!("jmp"); } },
    }
    if bprint { println!(" =>    i={:<3} acc={}", i, acc); }
    if i == len { terminates = true; break; }
  }

  (executed.len(), i, acc, terminates)
}