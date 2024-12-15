use std::collections::HashMap;

use crate::{components::{Drawable, Map2D}, shared::report_progress};

pub struct HikeMap {
  grid: Vec<Vec<char>>,
  pub trailheads: HashMap<(usize, usize), HashMap<(usize, usize), u32>>,
}
impl HikeMap {
  pub fn new(str_ref: &String) -> HikeMap {
    HikeMap {
      grid: HikeMap::build_from_char_text(str_ref),
      trailheads: HashMap::new(),
    }
  }

  fn search_for_head(&mut self, (x, y): (usize, usize), original: &(usize, usize)) {
    let curr_value = self.get(&(x, y)).unwrap().to_digit(10).unwrap();
    for value in ["top", "right", "bot", "left"].map(|dir| self.get_at_dir(&(x, y), dir)) {
      if value.is_none() { continue; };
      let (next_x, next_y, next_char) = value.unwrap();
      let next_value = next_char.to_digit(10).unwrap();

      if curr_value == 8 && next_value == 9 {
        let trailhead = self.trailheads.get_mut(original).unwrap();
        let next = (next_x, next_y);
        trailhead.insert(next, trailhead.get(&next).unwrap_or(&0) + 1);
        continue;
      }

      if next_value == curr_value + 1 {
        self.search_for_head((next_x, next_y), original);
      }
    }
  }

  pub fn find_trailheads(&mut self) {
    let rows = self.grid.len();
    let cols = self.grid[0].len();

    let mut current = 0;
    let total = rows * cols + 1;

    for y in 0..rows {
      for x in 0..cols {
        // track
        report_progress(current, total);
        current += 1;

        let curr_value = self.get(&(x, y)).unwrap().to_digit(10).unwrap();
        if curr_value != 0 { continue; }

        self.trailheads.insert((x, y), HashMap::new());
        self.search_for_head((x, y), &(x, y));
      }
    }
    println!();
  }
}
impl Map2D for HikeMap {
  fn grid_map(&self) -> &Vec<Vec<char>> {
    &self.grid
  }
}
impl Drawable for HikeMap {}
