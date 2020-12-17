use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day13-example.dat")?;
  let lines: Vec<String> = BufReader::new(file).lines()
                           .map(|l| l.unwrap()).collect();
  //let _timestamp: u64 = lines[0].parse().unwrap();

  //part1(&lines, timestamp);
  part2(&lines);

  Ok(())
}

#[allow(dead_code)]
fn part1(lines: &Vec<String>, timestamp: u64) {
  let busses: Vec<u64> = lines[1].split(',')
                         .filter(|&s| s != "x")
                         .map(|s| s.parse().unwrap())
                         .collect();

  let departs: Vec<(u64, u64)> = busses.iter().map(|&x| (x, x * ((timestamp / x) + 1))).collect();
  println!("Departs: ({}) {:?}", timestamp, &departs);

  // Both numbers are relevant, so I can't simply use .min() unfortunately
  let (mut early_b, mut early_t) = departs[0];
  for &(x, t) in &departs[1..] {
    if t < early_t {
      early_b = x;
      early_t = t;
    }
  }

  println!("{} * ({} - {}) = {}", early_b, timestamp, early_t, early_b * (early_t - timestamp));
}

fn part2(lines: &Vec<String>) {
  let mut busses = lines[0].split(',')
                   .enumerate()
                   .filter(|&(_, s)| s != "x")
                   .map(|(t, s)| (t as u128, s.parse().unwrap()))
                   .collect::<Vec<(u128, u128)>>();
  busses.sort_by(|(_, a), (_, b)| a.cmp(b));
  println!("{:?}", busses);
  println!("I activate my secret math card: the CHINESE REMAINDER THEOREM!!");
  println!("Calculating...");
  let chinese_timestamp = chinese_remainder(&busses);
  println!("Found the answer: \x1b[33m\x1b[1m{:?}\x1b[0m", chinese_timestamp);
}

fn gcd(a: u128, b: u128) -> u128 {
  if a == 0 { b }
  else { gcd(b % a, a) }
}

fn egcd(a: u128, b: u128) -> (u128, u128, u128) {
  if a == 0 { (b, 0, 1) }
  else {
    let (q, s, t) = egcd(b % a, a);
    (q, t - s * b / a, s)
  }
}

// Welp, for part 2, I had no clue, so I looked into the reddit to see what people did.
// And from what I saw, I'd say that I didn't give up, I fucking LEARNED something. <_<
// Chinese Remainder Theorem: https://crypto.stanford.edu/pbc/notes/numbertheory/crt.html
/// Note: returns None if the vec of pairs is empty or if the moduli are not prime to each other.
fn chinese_remainder(pairs: &Vec<(u128, u128)>) -> Option<u128> {
  // Check if the moduli are prime to each other
  let mut ipairs = pairs.iter();
  let (_, mut prev_id) = ipairs.next()?;
  for &(_, curr_id) in ipairs {
    if gcd(prev_id, curr_id) != 1 { return None; }
    prev_id = curr_id;
  }

  let sumod: u128 = pairs.iter().map(|&(_a, m)| m).product();
  let x: u128 = pairs.iter().map(|&(a, m)| {
    let b = sumod / m;
    let (_, _, s) = egcd(m, b);
    a * s * b
  }).sum();
  Some(x % sumod)
}
