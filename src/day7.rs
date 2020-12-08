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
      Some((
        iter.next().unwrap().parse::<u32>().unwrap(),
        iter.next().unwrap().rsplitn(2, " ").nth(1).unwrap()
      ))
    }).collect();
    (a, b)
  }).collect();
  
  let mut temp_hs = HashSet::new();
  assert!(parts.iter().all(|&(k, _)| temp_hs.insert(k)));
  let hashmap: HashMap<&str, HashSet<(u32, &str)>> = parts.into_iter().collect();
  println!("{:?}", hashmap);
  part2(&hashmap);
  Ok(())
}

fn part1(hashmap: HashMap<&str, HashSet<&str>>) {
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
}

fn part2(hashmap: &HashMap<&str, HashSet<(u32, &str)>>) {
  let shiny_gold = hashmap.get(&"shiny gold").unwrap();
  println!("shiny gold: {:?}\n", shiny_gold);
  
  let tree: HashTree = set2tree(1, shiny_gold, hashmap);
  println!("The tree: {:?}", tree);
  println!("\nThe fold: {}", fold_tree(&tree) - 1);
  println!("BEAUTIFUL");
}

#[derive(Debug)]
enum HashTree { Leaf(u32), Node(u32, Vec<HashTree>) }
fn set2tree(
  n: u32,
  hashset: &HashSet<(u32, &str)>,
  hashmap: &HashMap<&str, HashSet<(u32, &str)>>
) -> HashTree {
  if hashset.is_empty() { return HashTree::Leaf(n); }
  let sets: Vec<(u32, &HashSet<(u32, &str)>)> = hashset.iter()
  .map(|(n, s)| (*n, hashmap.get(s).unwrap()))
  .collect();
  HashTree::Node(n, sets.iter().map(|(k, hs)| set2tree(*k, hs, hashmap)).collect())
}

fn fold_tree(hashtree: &HashTree) -> u32 {
  match hashtree {
    HashTree::Leaf(n) => *n,
    HashTree::Node(n, v) => {
      let sum: u32 = v.iter().map(fold_tree).sum();
      n + n * sum
    }
  }
}