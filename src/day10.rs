#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day10.dat")?;
  let mut numbers: Vec<usize> = BufReader::new(file).lines()
                                .map(|l| l.unwrap().parse().unwrap()).collect();
  numbers.sort();
  let mut groups = vec![vec![0]];
  //let mut differences: Vec<u32> = vec![0; numbers.len()];
  let mut prev = 0;
  for &n in &numbers {
    match n - prev {
      3 => groups.push(vec![n]),
      1 => groups.last_mut().unwrap().push(n),
      _ => panic!("Panik!"),
    }
    prev = n;
  }

  let group_arrangements: Vec<_> = groups.iter().map(|g| ar_range_ments(g.len(), 0)).collect();
  println!("{:?}", groups);
  println!("{:?}", group_arrangements);
  let answer: u128 = group_arrangements.iter().product();
  println!("The answer is {}", answer);
  
  for i in 1..=20 {
    println!("{:<3} -> {}", i, ar_range_ments(i, 0));
  }

  Ok(())
}

// Managed to find this recursion
// Don't know why, but I have to check with n-1 instead of n
fn ar_range_ments(n: usize, acc: usize) -> u128 {
  if n-1 == acc { 1 }
  else if n-1 < acc { 0 }
  else { ar_range_ments(n, acc + 1)
       + ar_range_ments(n, acc + 2)
       + ar_range_ments(n, acc + 3) }
}

/* Long sketching for the ar_range_ments function
0 1 2       | 1 1
0 2         | 2

0 1 2 3     | 1 1 1
0 1 3       | 1 2
0 2 3       | 2 1
0 3         | 3

0 1 2 3 4   | 1 1 1 1
0 1 2 4     | 1 1 2
0 1 3 4     | 1 2 1
0 1 4       | 1 3
0 2 3 4     | 2 1 1
0 2 4       | 2 2
0 3 4       | 3 1

0 1 2 3 4 5 | 1 1 1 1 1
0 1 2 3 5   | 1 1 1 2
0 1 2 4 5   | 1 1 2 1
0 1 2 5     | 1 1 3
0 1 3 4 5   | 1 2 1 1
0 1 3 5     | 1 2 2
0 1 4 5     | 1 3 1
0 2 3 4 5   | 2 1 1 1
0 2 3 5     | 2 1 2
0 2 4 5     | 2 2 1
0 2 5       | 2 3
0 3 4 5     | 3 1 1
0 3 5       | 3 2
*/
