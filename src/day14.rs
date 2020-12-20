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


impl Inst {
  fn string_to_mask(ms: String) -> Option<Self> {
    ms.chars().enumerate().fold(
      Some(Inst::Mask((1 << 36) - 1, 0)),
      |m, (i, c)| match m {
        Some(Inst::Mask(mut mand, mut mor)) => {
          let b = 1 << (35 - i as u64);
          match c {
            '1' => mor |= b,
            '0' => mand &= !b,
            'X' => (),
             _  => return None,
          }
          Some(Inst::Mask(mand, mor))
        },
        _ => None,
      }
    )
  }

  fn mask_to_string(&self) -> Option<String> {
    match self {
      Inst::Mask(mand, mor) => {
        let mut ms = vec!['X'; 36];
        for i in 0..36 {
          let b = 1 << (35 - i);
          if mand & b == 0 { ms[i] = '0'; }
          if mor & b > 0 { ms[i] = '1'; }
        }
        Some(ms.into_iter().collect())
      },
      _ => None,
    }
  }
}

impl fmt::Display for Inst {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Inst::Mask(mand, mor) =>
        write!(f, "{}Mask{} {}{}{}\n  {}&& {:036b}{}\n  {}|| {:036b}{}",
               B, C, BLU, self.mask_to_string().unwrap(), C, YEL, mand, C, RED, mor, C),
      Inst::Mem(index, value) =>
        write!(f, "{}Mem{}[{}{:08x}{}] = {}{:08x}{}",
               B, C, GRN, index, C, MAG, value, C),
    }
  }
}

fn execute(insts: &Vec<Inst>) -> HashMap<u64, u64> {
  let mut memory: HashMap<u64, u64> = HashMap::new();
  let (mut mand, mut mor) = (0, 0);
  for &inst in insts {
    match inst {
      Inst::Mask(mandi, mori) => { mand = mandi; mor = mori; },
      Inst::Mem(index, value) => { memory.insert(index, value & mand | mor); }
    }
  }

  memory
}

#[allow(dead_code)]
fn part1(data: &Vec<Inst>) {
  let memory = execute(data);
  let mut k = 0;
  println!("{}{:=^143}{}", B, " MEMORY ", C);
  for (&i, &v) in &memory {
    if k != 0 && k % 7 == 0 { println!(" |"); }
    print!(" | {}{:04x} {}:: {}{:09x}{}", GRN, i, RED, YEL, v, C);
    k += 1;
  }
  println!(" |");
  println!("{}{:=^143}{}", B, " END OF MEMORY ", C);

  let answer: u64 = memory.iter().map(|(_, &v)| v).sum();
  println!("\nMEMORY COLLAPSED: answer is {}{}{}", B, answer, C);
}

//////////////
/// Part 2 ///
//////////////

#[derive(Debug, Clone)]
enum Inst2 {
  /// BASE, FLOATING_INIT and FLOATING.
  Mask(u64, u64, Vec<usize>),
  Mem(u64, u64),
}

impl Inst2 {
  fn string_to_mask(ms: String) -> Option<Self> {
    let mut base = 0;
    let mut floating_init = (1 << 36) - 1;
    let mut floatings = Vec::new();

    for (i, c) in ms.chars().enumerate() {
      let j = 35 - i;
      let b = 1 << j;
      match c {
        '0' => (),
        '1' => base |= b,
        'X' => {
          floatings.push(j);
          floating_init &= !b;
        },
         _  => return None,
      }
    }

    Some(Inst2::Mask(base, floating_init, floatings))
  }
  
  fn mask_to_string(&self) -> Option<String> {
    match self {
      Inst2::Mask(base, _, floatings) => {
        let s = format!("{:036b}", base);
        let mut v: Vec<char> = s.chars().collect();
        for &floating in floatings { v[35 - floating] = 'X'; }
        Some(v.into_iter().collect())
      },
      _ => None,
    }
  }

  fn decode_addresses(&self, a: u64) -> Option<Vec<u64>> {
    match self {
      Inst2::Mask(base, floating_init, floatings) => {
        let base_address = a & floating_init | base;
        let mut addresses = Vec::new();
        
        // Found this clever way to get all combinations possible
        let flen = floatings.len();
        for i in 0..(1 << flen) {
          let mut address = base_address;
          for j in 0..flen {
            if i & (1 << j) != 0 {
              address += 1 << floatings[j];
            }
          }
          addresses.push(address);
        }
        
        // The sorting isn't really useful for the algorithms
        // but at least nice for debugging and pretty-printing
        addresses.sort();
        Some(addresses)
      },
      _ => None,
    }
  }
}

impl fmt::Display for Inst2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Inst2::Mask(base, floating_init, _) =>
        write!(f, "{}Mask{} {}{}{}\n  {}_B {:036b}{}\n  {}XX {:036b}{}",
               B, C, BLU, self.mask_to_string().unwrap(), C,
               RED, base, C, YEL, floating_init, C),
      Inst2::Mem(index, value) =>
        write!(f, "{}Mem{}[{}{:08x}{}] = {}{:08x}{}",
               B, C, GRN, index, C, MAG, value, C),
    }
  }
}

fn execute2(insts: &Vec<Inst2>) -> HashMap<u64, u64> {
  let mut memory: HashMap<u64, u64> = HashMap::new();
  let mut base = 0;
  let mut floating_init = (1 << 36) - 1;
  let mut floatings = Vec::new();
  for inst in insts {
    // Hmmm... There's a cloning of the vectors going on, and I don't know how to safely avoid that...
    match inst {
      Inst2::Mask(ibase, ifloating_init, ifloatings) => {
        base = *ibase;
        floating_init = *ifloating_init;
        floatings = ifloatings.to_vec();
      },
      Inst2::Mem(index, value) => {
        let mask = Inst2::Mask(base, floating_init, floatings.clone());
        if let Some(addresses) = mask.decode_addresses(*index) {
          for a in addresses { memory.insert(a, *value); }
        }
      }
    }
  }

  memory
}

fn part2(data: &Vec<Inst2>) {
  for inst in data {
    println!("{}", inst);
  }
  
  let memory = execute2(data);
  let mut k = 0;
  println!("{}{:=^147}{}", B, " MEMORY ", C);
  for (&i, &v) in &memory {
    if k != 0 && k % 6 == 0 { println!(" |"); }
    print!(" | {}{:09x} {}:: {}{:08x}{}", GRN, i, RED, YEL, v, C);
    k += 1;
  }
  println!(" |");
  println!("{}{:=^147}{}", B, " END OF MEMORY ", C);

  let answer: u64 = memory.iter().map(|(_, &v)| v).sum();
  println!("\nMEMORY COLLAPSED: answer is {}{}{}", B, answer, C);

  // That test worked :D
  //let mask = Inst2::string_to_mask("00000000000000000000000000000000X0XX".to_string()).unwrap();
  //println!("\n\n{}", mask);
  //println!("{:?}", mask.decode_addresses(26));
}

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day14.dat")?;
  let data: Vec<_> = BufReader::new(file).lines()
    .map(|l| {
      let l = l.unwrap();
      let split = l.split(" = ").collect::<Vec<_>>();
      let (left, right) = (split[0].to_string(), split[1].to_string());
      if left == "mask" {
        Inst2::string_to_mask(right).unwrap()
      } else {
        let index: u64 = left[4..left.len()-1].parse().unwrap();
        let value: u64 = right.parse().unwrap();
        Inst2::Mem(index, value)
      }
    }).collect();

  //part1(&data);
  part2(&data);

  Ok(())
}
