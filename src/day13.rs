use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day13.dat")?;
  let lines: Vec<String> = BufReader::new(file).lines()
                           .map(|l| l.unwrap()).collect();
  let _timestamp: u64 = lines[0].parse().unwrap();

  //part1(&lines, timestamp);
  part2(&lines[1]);

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

fn part2(line: &String) {
  let mut busses = line.split(',')
                   .enumerate()
                   .filter(|&(_, s)| s != "x")
                   .map(|(t, s)| (t as u128, s.parse().unwrap()))
                   .collect::<Vec<(u128, u128)>>();
  busses.sort_by(|(a, _), (b, _)| a.cmp(b));
  println!("{:?}", busses);
  println!("Calculating...");
  let _ = incremental_computation(&busses);
}

fn gcd(a: u128, b: u128) -> u128 {
  if a == 0 { b }
  else { gcd(b % a, a) }
}

fn lcm(a: u128, b: u128) -> u128 {
  a * b / gcd(a, b)
}

fn earliest_timestamp(id: u128, t: u128) -> u128 {
  (id - t % id) % id
}

// Based off of an explanation of some trick on Reddit -_-'
fn incremental_computation(busses: &Vec<(u128, u128)>) -> u128 {
  let (_, ida) = busses[0];
  let mut last_timestamp = 0; // start at the first offset of the first bus
  let mut q = ida;
  let mut stdout = io::stdout();
  for &(ob, idb) in &busses[1..] {
    while earliest_timestamp(idb, last_timestamp) != ob % idb {
      last_timestamp += q;
      print!("\r{:?} {}", (ob, idb), last_timestamp);
      stdout.flush().unwrap();
    }
    q = lcm(q, idb);
    println!();
  }
  let ets: Vec<_> = busses.iter()
                    .map(|&(o, id)| (o, earliest_timestamp(id, last_timestamp), id))
                    .collect();
  println!("Answer found: \x1b[1m{}\x1b[0m", last_timestamp);
  println!("Offsets: {:?}", ets);
  
  last_timestamp
}

// Nothing of this down here worked. I don't know why I'm just never getting the right answer.
// Surely, of course, I've got something wrong in the calculus.
// I guess I didn't actually learn that much unfortunately. :/
/*
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
*/
