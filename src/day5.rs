use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day5.dat")?;
  let lines = BufReader::new(file).lines();
  println!("{:?}", lines);

  let data: Vec<(u8, u8)> = lines.map(|rl| {
    if let Ok(l) = rl {
      let (a, b) = l.split_at(7);
      (ab_to_num(a.into(), 'F', 'B'), ab_to_num(b.into(), 'L', 'R'))
    } else { panic!("wtf, there's a line error somewhere") }
  }).collect();
  
  let mut ids: Vec<u32> = data.iter()
    .map(|(row, col)| *row as u32 * 8 + *col as u32).collect();
  ids.sort();
  let mut last: u32 = ids[0] - 1;
  for &id in &ids {
    if id != last + 1 { continue }
    last += 1;
  }
  
  println!("{:?}", ids);
  println!("Max: {:?} | My seat: {}", ids.iter().max(), last + 1);

  Ok(())
}

fn ab_to_num(fbs: String, a: char, b: char) -> u8 {
  let numvec: Vec<u8> = fbs.chars().rev().map(|c|
    if c == a { 0 }
    else if c == b { 1 }
    else { panic!("wtf there's another character than F and B ?!?!") }
  ).collect();
  let (mut idx, mut res): (usize, u8) = (0, 0);
  for b in numvec {
    res += b << idx;
    idx += 1;
  }

  res
}

