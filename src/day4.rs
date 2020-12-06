use std::fs::read_to_string;
use std::io::Result;

pub fn main() -> Result<()> {
  let raw = read_to_string("resources/day4.dat")?;
  let passports: Vec<Vec<(&str, &str)>> = raw.split("\n\n")
    .map(|s| s.split_whitespace()
      .map(|s| { let mut s = s.split(":");
        (s.next().unwrap(), s.next().unwrap()) })
      .collect()).collect();
  
  let validity: Vec<bool> = passports
    .iter().map(check_passport).collect();
  
  let valids = validity.iter().fold(0, |acc, &b| acc + if b {1} else {0});
  println!("{:#?}", passports);
  println!("There are \x1b[1m{}\x1b[0m valid passwords (hacc)", valids);
  Ok(())
}

fn check_passport(passport: &Vec<(&str, &str)>) -> bool {
  let fields: Vec<&str> = passport.iter().map(|(a, _)| *a).collect();
  for valid_field in vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
    if !fields.contains(&valid_field) { return false; }
  }
  true
}

fn digits(l: usize, v: &str) -> Option<u32> {
  if v.len() != l { return None; }
  if let Ok(n) = v.parse::<u32>() { Some(n) }
  else { None }
}

fn nrange(v: &str, l: usize, min: u32, max: u32) -> bool {
  if let Some(n) = digits(l, v) {
    n >= min && n <= max
  } else { false }
}

fn byr(v: &str) -> bool { nrange(v, 4, 1920, 2002) }
fn iyr(v: &str) -> bool { nrange(v, 4, 2010, 2020) }
fn eyr(v: &str) -> bool { nrange(v, 4, 2020, 2030) }
