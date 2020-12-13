#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell { Empty, Full, NoSeat }
type Grid = Vec<Vec<Cell>>;

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", match self {
      Cell::Empty  => "\x1b[2m[]\x1b[0m",
      Cell::Full   => "\x1b[1m::\x1b[0m",
      Cell::NoSeat => "  ",
    })
  }
}

// ─│┌┐└┘
fn print_grid(grid: &Grid) {
  let w = if grid.is_empty() { 0 } else { grid[0].len() };
  println!("┌{}┐", "──".repeat(w));
  for line in grid {
    print!("│");
    for cell in line { print!("{}", cell); }
    println!("│");
  }
  println!("└{}┘", "──".repeat(w));
}

fn cell(grid: &Grid, x: isize, y: isize) -> Option<&Cell> {
  if x < 0 || y < 0 { None }
  else { grid.get(y as usize)?.get(x as usize) }
}

fn val(c: Option<&Cell>) -> u32 {
  match c { Some(&Cell::Full) => 1, _ => 0 }
}

fn neighbors1(grid: &Grid, x: usize, y: usize) -> u32 {
  let mut counter = 0;
  let (x, y) = (x as isize, y as isize);
  for j in y - 1 ..= y + 1 {
    for i in x - 1 ..= x + 1 {
      if (i, j) == (x, y) { continue }
      let cellthing = cell(grid, i, j);
      counter += val(cellthing);
    }
  }
  counter
}

fn neighbors2(grid: &Grid, x: usize, y: usize) -> u32 {
  let mut counter = 0;
  let (x, y) = (x as isize, y as isize);
  for j in y - 1 ..= y + 1 {
    for i in x - 1 ..= x + 1 {
      if (i, j) == (x, y) { continue }
      let mut d = 1;
      loop {
        let cellthing = cell(grid, i*d, j*d);
        counter += val(cellthing);
        match cellthing {
          Some(&Cell::NoSeat) => d += 1,
          _ => break,
        }
      }
    }
  }
  counter
}

fn grid_deepcopy(grid: &Grid) -> Grid {
  grid.iter().map(|v| v.clone().into_iter().collect()).collect()
}

fn automata(grid: &mut Grid) -> (u32, bool) {
  let mut newgrid: Grid = grid_deepcopy(grid);
  for j in 0 .. newgrid.len() {
    let newline = &mut newgrid[j];
    for i in 0 .. newline.len() {
      if newline[i] == Cell::NoSeat { continue }
      //let neighbors = neighbors1(grid, i, j);
      let neighbors = neighbors2(grid, i, j);
      if neighbors >= 4 { newline[i] = Cell::Empty; }
      else if neighbors == 0 { newline[i] = Cell::Full; }
    }
  }
  let (mut seats, mut changed) = (0, false);
  for j in 0 .. newgrid.len() {
    let newline = &mut newgrid[j];
    for i in 0 .. newline.len() {
      if newline[i] == Cell::Full { seats += 1; }
      if grid[j][i] != newline[i] {
        grid[j][i] = newline[i];
        changed = true;
      }
    }
  }
  (seats, changed)
}

pub fn main() -> io::Result<()> {
  let file = File::open("resources/day11.dat")?;
  let mut grid: Grid = BufReader::new(file).lines()
    .map(|l| l.unwrap().chars()
      .map(|c| match c {
        'L' => Cell::Empty,
        '#' => Cell::Full,
        '.' => Cell::NoSeat,
         _  => panic!("Data is corrupted!"),
      }).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
  
  let mut running = true;
  let mut seats = 0;
  print_grid(&grid);
  while running {
    let (s, r) = automata(&mut grid);
    print_grid(&grid);
    println!("{} seats, {}running", s, if r { "" } else { "not " });
    running = r; seats = s;
    std::thread::sleep_ms(64);
  }

  Ok(())
}
