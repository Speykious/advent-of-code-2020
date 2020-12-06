#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn main() -> Result<()> {
  let file = File::open("resources/day1.dat")?;
  let lines = BufReader::new(file).lines();
  let mut numbers: Vec<u64> = Vec::new();
  for line in lines {
    numbers.push(line?.parse().unwrap());
  }
  println!("Finding two numbers summing up to 2020...");
  let (a, b) = puzzle1(&numbers);
  println!("Found {} + {} == {}, summing up to {}", a, b, a + b, a * b);
  println!("Finding three numbers summing up to 2020...");
  let (a, b, c) = puzzle2(&numbers);
  println!("Found {} + {} + {} == {}, summing up to {}", a, b, c, a + b + c, a * b * c);

  Ok(())
}

// Double brute-force
fn puzzle1(xs: &Vec<u64>) -> (u64, u64) {
  let (mut a, mut b);
  for j in 0..(xs.len() - 1) {
    a = xs[j];
    for i in (j+1)..xs.len() {
      b = xs[i];
      if a + b == 2020 {
        return (a, b);
      }
    }
  }
  (0, 0)
}

// Triple brute-force
fn puzzle2(xs: &Vec<u64>) -> (u64, u64, u64) {
  let (mut a, mut b, mut c);
  for k in 0..(xs.len() - 2) {
    a = xs[k];
    for j in (k+1)..(xs.len() - 1) {
      b = xs[j];
      for i in (j+1)..xs.len() {
        c = xs[i];
        if a + b + c == 2020 {
          return (a, b, c);
        }
      }
    }
  }
  (0, 0, 0)
}