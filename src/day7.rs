use std::fs::File;
use std::io::{self, BufRead, BufReader};
use nom::character::complete::{alpha1, space1, digit1};
use std::str::from_utf8;

fn utf8<'l>(v: &&'l [u8]) -> &'l str {
  from_utf8(v).unwrap()
}

enum Bag<'a, 'b, 'c> {
  SeparatedList(Vec<(&'a [u8], (Vec<&'b [u8]>, &'c [u8]))>),
  Tag(&'a [u8]),
}

type BagRule<'a, 'b> = Vec<(u8, Vec<&'a str>, &'b str)>;
pub fn main() -> io::Result<()> {
  let file = File::open("resources/day7.dat")?;
  let mut borrowed_lines = Vec::new();
  for line in BufReader::new(file).lines() {
    borrowed_lines.push(line?);
  }
  // let lines = borrowed_lines.clone();

  named!(rule<((Vec<&[u8]>, &[u8]), Vec<(&[u8], (Vec<&[u8]>, &[u8]))>)>, do_parse!(
    color_bc: many_till!(
      terminated!(alpha1, space1),
      tag!("bags contain ")
    ) >>
    bags: alt!(
      separated_list1!(
        terminated!(char!(','), space1),
        do_parse!(
          n: terminated!(digit1, space1) >>
          color_b: many_till!(
            terminated!(alpha1, space1),
            alt!(tag!("bags") | tag!("bag"))
          ) >>
          (n, color_b)
        )
      ) | tag!("no other bags")
    ) >>
    char!('.') >>
    eof!() >>
    (color_bc, bags)
  ));
  /*
  let mut rules: Vec<BagRule> = Vec::new();
  for line in lines {
    let line: Vec<u8> = line.clone().into_bytes();
    let (_, ((_, _), bags)) = rule(&line).unwrap();
    //let color: Vec<&str> = color.iter().map(utf8).collect();
    let bags: BagRule = bags.iter().map(|(a, (v, b))| (
      utf8(a).parse::<u8>().unwrap(), v.iter().map(utf8).collect(), utf8(b))).collect();
    rules.push(bags);
  }
  */
  let rules: Vec<BagRule> = borrowed_lines.iter()
  .map(|line| {
    if let Ok((_, ((_, _), bags))) = rule(line.as_bytes()) {
    bags.iter().map(|(a, (v, b))| (
      utf8(a).parse::<u8>().unwrap(),
      v.into_iter().map(utf8).collect(),
      utf8(b)
    )).collect()
    } else {
      panic!(format!("PANIK -> {:?}", line))
    }
  }).collect();

  println!("{:?}", rules);
  Ok(())
}