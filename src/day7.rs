use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::{HashSet, HashMap};

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day7.dat")?;
  let mut lines = Vec::new();
  for line in BufReader::new(file).lines() {
    lines.push(line?);
  }
  let parts: Vec<_> = lines.iter()
  .map(|line| {
    let mut iter = line.split(" bags contain ");
    let (a, b) = (iter.next().unwrap(), iter.next().unwrap());
    //let a: String = a.into();
    let b: Vec<_> = b.split(", ").collect();
    let b: HashSet<_> = b.into_iter().filter_map(|s| {
      if s == "no other bags." { return None; }
      let mut iter = s.splitn(2, " ");
      Some(iter.nth(1).unwrap().rsplitn(2, " ").nth(1).unwrap())
    }).collect();
    (a, b)
  }).collect();
  let mut temp_hs = HashSet::new();
  assert!(parts.iter().all(|&(k, _)| temp_hs.insert(k)));
  let hashmap: HashMap<_, _> = parts.into_iter().collect();
  let shiny: HashMap<&str, _> = hashmap.iter()
  .filter(|(_, hs)| hs.contains(&"shiny gold"))
  .map(|(&a, b)| (a, b))
  .collect();
  let mut bagset: HashSet<_> = shiny.keys().map(|&k| k).collect();
  while hashmap.iter()
  .filter(|(_, hs)| !hs.intersection(&bagset).collect::<HashSet<_>>().is_empty())
  .collect::<HashMap<_, _>>().iter() // Solution du Pauvre
  .map(|(&k, _)| bagset.insert(k)).fold(false, |a, b| a || b) {}
  
  println!("{:#?}", bagset);
  println!("Total number of bags able to contain shiny gold: {}", bagset.len());
  println!("Total number of bags in the total hashmap: {}", hashmap.len());
  Ok(())
}