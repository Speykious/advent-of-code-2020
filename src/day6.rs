#![allow(dead_code)]
use std::fs::read_to_string;
use std::io;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug)]
struct Group(Vec<String>);
impl Group {
  pub fn is_common_answer(&self, c: char) -> bool {
    let Group(vec) = self;
    for s in vec {
      if !s.contains(c) {
        return false;
      }
    }
    true
  }

  pub fn common_answers(&self) -> String {
    let mut cas: String = "".into();
    for c in ALPHA.chars() {
      if self.is_common_answer(c) {
        cas.push(c);
      }
    }
    cas
  }

  pub fn is_answer(&self, c: char) -> bool {
    let Group(vec) = self;
    for s in vec {
      if s.contains(c) {
        return true;
      }
    }
    false
  }

  pub fn answers(&self) -> String {
    let mut cas: String = "".into();
    for c in ALPHA.chars() {
      if self.is_answer(c) {
        cas.push(c);
      }
    }
    cas
  }
}



pub fn main() -> io::Result<()> {
  let file = read_to_string("resources/day6.dat")?;
  let groups: Vec<Group> = file
    .split("\n\n")
    .map(|s| Group(s.lines().map(|s| s.into()).collect()))
    .collect();
  let common_answers_counts: Vec<usize> = groups
    .iter().map(|g| g.common_answers().len()).collect();
  
  println!("\x1b[33mCommon answers\x1b[0m\n{:?}", common_answers_counts);
  println!("\nTotal number of common answers: {}", common_answers_counts.iter().sum::<usize>());
  Ok(())
}