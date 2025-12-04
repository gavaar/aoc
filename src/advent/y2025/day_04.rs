use std::collections::HashSet;

use crate::{components::{Map2D, Map2DDirection}, shared::{Color, read_input}};

struct PrintingDpt {
  grid_map: Vec<Vec<char>>,
  removed_map: HashSet<(usize, usize)>,
}
impl PrintingDpt {
  pub fn new(input: &String) -> PrintingDpt {
    PrintingDpt {
      grid_map: PrintingDpt::build_from_char_text(input),
      removed_map: HashSet::new(),
    }
  }
}
impl Map2D for PrintingDpt {
  fn grid_map(&self) -> &Vec<Vec<char>> {
    &self.grid_map
  }
}

fn find_movable_rolls(map: &PrintingDpt) {
  let mut can_be_moved = 0;

  for row in 0..map.rows() {
    for col in 0..map.cols() {
      let current = (row, col);
      if map.get(&current) == Some(&'.') { continue }

      let roll_count = Map2DDirection::get_all_dirs()
        .iter()
        .map(|dir| map.get_at_dir(&current, &dir).unwrap_or((0,0,'.')))
        .filter(|(_r, _c, value)| value == &'@')
        .count();
      if roll_count < 4 {
        can_be_moved += 1;
      }
    }
  }
  
  println!("Rolls that can be moved: {}", Color::Green(can_be_moved));
}

fn remove_all_rolls(map: &mut PrintingDpt) {
  loop {
    let mut to_remove_values: HashSet<(usize, usize)> = HashSet::new();

    for x in 0..map.cols() {
      for y in 0..map.rows() {
        let current = (x, y);
        if map.get(&current).unwrap_or(&'.') == &'.' { continue; }

        let roll_count = Map2DDirection::get_all_dirs()
          .iter()
          .map(|dir| map.get_at_dir(&current, &dir).unwrap_or((0,0,'.')))
          .filter(|(_r, _c, value)| value == &'@')
          .count();
        if roll_count < 4 {
          to_remove_values.insert(current);
        }
      }
    }

    if to_remove_values.is_empty() { 
      break;
    }

    for removed in to_remove_values {
      let row_to_modify = map.grid_map.get_mut(removed.1).expect("should exist");
      row_to_modify[removed.0] = '.';

      map.removed_map.insert(removed);
    }
  }
}

pub fn run() {
  let test_input = read_input("day_04/test");
  let mut test_map = PrintingDpt::new(&test_input);
  find_movable_rolls(&test_map);
  remove_all_rolls(&mut test_map);
  println!("removed: {}", test_map.removed_map.len());

  println!();
  let input = read_input("day_04/input");
  let mut map = PrintingDpt::new(&input);
  find_movable_rolls(&map);
  remove_all_rolls(&mut map);
  println!("removed: {}", map.removed_map.len());
}
