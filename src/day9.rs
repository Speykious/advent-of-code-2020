use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day9.dat")?;
  let mut numbers: Vec<u64> = Vec::new();
  for line in BufReader::new(file).lines() {
    numbers.push(line?.parse().unwrap());
  }
  
  let preamble: Vec<u64> = numbers[..25].to_vec();
  let numbers = numbers[25..].to_vec();
  for n in numbers {
    if !is_sum_of_2_in(n, &preamble) {
      println!("The answer is {}", n);
      break;
    }
  }

  Ok(())
}

fn is_sum_of_2_in(n: u64, v: &Vec<u64>) -> bool {
  let vlen = v.len();
  for i in 0..vlen-1 {
    let a = v[i];
    for j in i+1..vlen {
      if a + v[j] == n {
        return true;
      }
    }
  }

  false
}