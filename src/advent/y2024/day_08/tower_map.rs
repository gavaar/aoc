use std::collections::{HashMap, HashSet};

use crate::{components::{Drawable, Map2D}, shared::Color};

pub struct TowerMap {
  grid: Vec<Vec<char>>,
  pub limit_to_1: bool,
  pub anti_node_map: HashSet<(usize, usize)>,
}
impl TowerMap {
  pub fn new(input: &String) -> TowerMap {
    TowerMap {
      limit_to_1: true,
      grid: TowerMap::build_from_char_text(input),
      anti_node_map: HashSet::new(),
    }
  }

  pub fn reset(&mut self) {
    self.anti_node_map = HashSet::new();
  }

  pub fn get_antinodes(&self, first: &(usize, usize), other: &(usize, usize)) -> Vec<(usize, usize)> {
    let diff_x = other.0 as isize - first.0 as isize;
    let diff_y = other.1 as isize - first.1 as isize;
    
    let mut antinodes = Vec::new();
    let mut first_antinode = (first.0, first.1);
    let mut other_antinode = (other.0, other.1);

    if !self.limit_to_1 {
      antinodes.push(first_antinode);
      antinodes.push(other_antinode);
    }

    loop {
      let ifirst_antinode = (first_antinode.0 as isize - diff_x, first_antinode.1 as isize - diff_y);

      if ifirst_antinode.0 < 0 || ifirst_antinode.1 < 0 {
        break;
      }

      first_antinode = (ifirst_antinode.0 as usize, ifirst_antinode.1 as usize);
      if self.get(&first_antinode).is_none() {
        break;
      }

      antinodes.push(first_antinode);

      if self.limit_to_1 {
        break;
      }
    }

    loop {
      let iother_antinode = (other_antinode.0 as isize + diff_x, other_antinode.1 as isize + diff_y);

      if iother_antinode.0 < 0 || iother_antinode.1 < 0 {
        break;
      }

      other_antinode = (iother_antinode.0 as usize, iother_antinode.1 as usize);

      if self.get(&other_antinode).is_none() {
        break;
      }

      antinodes.push(other_antinode);

      if self.limit_to_1 {
        break;
      }
    }

    antinodes
  }

  pub fn build_anti_nodes(&mut self) {
    let mut node_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (row, row_value) in self.grid.iter().enumerate() {
      for (col, land) in row_value.iter().enumerate() {
        if land == &'.' { continue; }

        let pos = (col, row);
        if node_map.contains_key(land) {
          node_map.get_mut(land).unwrap().push(pos);
        } else {
          node_map.insert(*land, vec![pos]);
        }
      }
    }

    for (_land, pos_list) in node_map.iter() {
      for x in 0..pos_list.len() {
        for y in 1 + x..pos_list.len() {
          let first_pos = pos_list.get(x).expect("cant break");
          let second_pos = pos_list.get(y).expect("cant break");

          for antinode in self.get_antinodes(first_pos, second_pos) {
            self.anti_node_map.insert(antinode);
          }
        }
      }
    }
  }
}

impl Map2D for TowerMap {
  fn grid_map(&self) -> &Vec<Vec<char>> {
    &self.grid
  }
}
impl Drawable for TowerMap {
  fn color_override(&self, value: impl std::fmt::Display, pos: &(usize, usize)) -> crate::shared::Color<impl std::fmt::Display> {
    if self.anti_node_map.contains(pos) {
      return Color::Blue(value);
    }

    Color::Default(value)
  }
}