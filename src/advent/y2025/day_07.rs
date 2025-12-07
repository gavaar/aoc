use std::collections::HashSet;

use crate::{
  components::{Map2D, Map2DDirection},
  shared::{Color, print_solution, print_test, read_input}
};

struct TachyonManifold {
  grid_map: Vec<Vec<char>>,
}
impl Map2D<char> for TachyonManifold {
  fn grid_map(&self) -> &Vec<Vec<char>> { &self.grid_map }
}
impl TachyonManifold {
  pub fn new(input: &String) -> TachyonManifold {
    let grid_map = Self::build_from_char_text(input);
    TachyonManifold { grid_map }
  }
}

fn part_one(tm: &TachyonManifold) {
  let start = tm.find(|unit| unit == &'S').expect("an S should be found");
  let mut splitted = 0;
  let mut beam_positions = HashSet::<(usize, usize)>::new();
  beam_positions.insert(start);

  loop {
    if beam_positions.is_empty() { break; }

    let mut new_positions = HashSet::new();

    for pos in beam_positions.drain() {
      let Some((next_x, next_y, next_val)) = tm.get_at_dir(&pos, &Map2DDirection::S) else {
        continue;
      };

      if next_val == '.' {
        new_positions.insert((next_x, next_y));
      }
      if next_val == '^' {
        if let Some((west_x, west_y, _)) = tm.get_at_dir(&(next_x, next_y), &Map2DDirection::W) {
          new_positions.insert((west_x, west_y));
        }
        if let Some((east_x, east_y, _)) = tm.get_at_dir(&(next_x, next_y), &Map2DDirection::E) {
          new_positions.insert((east_x, east_y));
        }
        splitted += 1;
      }
    }

    beam_positions = new_positions;
  }

  println!("The beam splitted {} times", Color::Red(splitted));
}

pub fn run() {
  print_test();
  let tachyon_manifold = TachyonManifold::new(&read_input("day_07/test"));
  part_one(&tachyon_manifold);

  println!();

  print_solution();
  let tms = TachyonManifold::new(&read_input("day_07/input"));
  part_one(&tms);
}
