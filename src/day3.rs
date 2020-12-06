#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day3.dat")?;
  let lines = BufReader::new(file).lines();
  let arr: Vec<Vec<char>> = lines.map(|line| line.unwrap().chars().collect()).collect();
  let (a, b, c, d, e) = (trees(&arr, 1, 1),
                         trees(&arr, 3, 1),
                         trees(&arr, 5, 1),
                         trees(&arr, 7, 1),
                         trees(&arr, 1, 2));
  println!("\nTotal Multiplication: {}", a*b*c*d*e);
  Ok(())
}

fn trees(arr: &Vec<Vec<char>>, dx: usize, dy: usize) -> u64 {
  let mut treecount = 0;
  let (mut j, mut i, lenj) = (0, 0, arr.len());
  while j < lenj {
    let leni = arr[j].len();
    if arr[j][i] == '#' {
      treecount += 1;
    }
    
    i = (i + dx) % leni;
    j += dy;
  }

  println!("Number of trees encountered for ({}, {}): \x1b[1m{}\x1b[0m", dx, dy, treecount);
  treecount
}