use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Move = (char, i32);
#[derive(Debug)]
struct Ship {
  wp: (i32, i32),
  x: i32,
  y: i32,
}

impl Ship {
  pub fn new() -> Self {
    Self::with_waypoint((1, 0))
  }

  pub fn with_waypoint(wp: (i32, i32)) -> Self {
    Self {
      wp,
      x: 0,
      y: 0,
    }
  }

  fn move_left(&mut self, a: i32) {
    let (dx, dy) = self.wp;
    self.wp = match (360 - a) % 360 {
       90 => (-dy, dx),
      180 => (-dx, -dy),
      270 => (dy, -dx),
        _ => (dx, dy),
    }
  }
  fn move_right(&mut self, a: i32) {
    let (dx, dy) = self.wp;
    self.wp = match (360 - a) % 360 {
       90 => (dy, -dx),
      180 => (-dx, -dy),
      270 => (-dy, dx),
        _ => (dx, dy),
    }
  }

  fn move_wp_north(&mut self, n: i32) {
    println!("The waypoint moves by {} units!", n);
    let (dx, dy) = self.wp;
    self.wp = (dx, dy-n);
  }
  fn move_wp_south(&mut self, n: i32) {
    println!("The waypoint moves by {} units!", n);
    let (dx, dy) = self.wp;
    self.wp = (dx, dy+n);
  }
  fn move_wp_east(&mut self, n: i32) {
    println!("The waypoint moves by {} units!", n);
    let (dx, dy) = self.wp;
    self.wp = (dx+n, dy);
  }
  fn move_wp_west(&mut self, n: i32) {
    println!("The waypoint moves by {} units!", n);
    let (dx, dy) = self.wp;
    self.wp = (dx-n, dy);
  }
  fn move_to_wp(&mut self, n: i32) {
    let (dx, dy) = self.wp;
    self.x += n*dx; self.y += n*dy;
  }

  pub fn move_ship(&mut self, &(c, n): &Move) {
    match c {
      'N' => self.move_wp_north(n),
      'S' => self.move_wp_south(n),
      'E' => self.move_wp_east(n),
      'W' => self.move_wp_west(n),
      'L' => { println!("The ship turns left!"); self.move_left(n) },
      'R' => { println!("The ship turns right!"); self.move_right(n) },
      'F' => self.move_to_wp(n),
       _  => panic!("Data corrupted!"),
    }
    println!("{:?}", self);
  }

  pub fn print_position(&self) {
    let (x, y) = (self.x, self.y);
    let (a, b) = (x.abs(), y.abs());
    println!("Ship is at {} {}, {} {}",
      a, if x < 0 { "west" } else { "east" },
      b, if y < 0 { "south" } else { "north" },
    );
    println!("Manhattan distance: {}", a + b);
  }
}

fn parse_move(s: String) -> Move {
  let mut s = s.chars();
  let c = s.next().unwrap();
  let n = s.collect::<String>().parse().unwrap();
  (c, n)
}

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day12.dat")?;
  //let moves: Vec<Move> = "F10\nN3\nF7\nR90\nF11".to_string().lines()
  //                      .map(|l| parse_move(l.to_string())).collect();
  let moves: Vec<Move> = BufReader::new(file).lines()
                        .map(|l| parse_move(l.unwrap())).collect();
  // Part 1
  let mut ship = Ship::new();
  for m in &moves { ship.move_ship(m); }
  println!();
  ship.print_position();
  println!();

  // Part 2
  let mut ship = Ship::with_waypoint((10, -1));
  for m in &moves { ship.move_ship(m); }
  println!();
  ship.print_position();
  println!();

  Ok(())
}
