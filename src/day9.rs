#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day9.dat")?;
  let mut numbers: Vec<u64> = Vec::new();
  for line in BufReader::new(file).lines() {
    numbers.push(line?.parse().unwrap());
  }
  
  let invalid = find_invalid(&numbers).unwrap();
  println!("Invalid number: {}", invalid);
  let range = find_range_sum(invalid, &numbers);
  println!("Range summing up to {}: {:?}", invalid, range);
  println!("Found weakness: {}", range.iter().min().unwrap() + range.iter().max().unwrap());

  Ok(())
}

fn is_sum_of_2_in(idx: usize, v: &Vec<u64>) -> bool {
  if idx < 25 { panic!("i has to be >= 25"); }
  let n = v[idx];
  let preamble = v[idx-25..idx].to_vec();
  let plen = preamble.len();
  for i in 0..plen-1 {
    let a = preamble[i];
    for j in i+1..plen {
      if a + preamble[j] == n {
        return true;
      }
    }
  }
  false
}

fn find_invalid(v: &Vec<u64>) -> Option<u64> {
  for i in 25..v.len() {
    if !is_sum_of_2_in(i, v) {
      return Some(v[i]);
    }
  }
  None
}

fn find_range_sum(n: u64, v: &Vec<u64>) -> Vec<u64> {
  let vlen = v.len();
  for i in 0..vlen-1 {
    for j in i+1..vlen {
      let range = v[i..j].to_vec();
      if range.iter().sum::<u64>() == n {
        return range;
      }
    }
  }
  
  Vec::new()
}