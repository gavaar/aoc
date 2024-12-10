use std::collections::{HashMap, HashSet};

use crate::{components::{Drawable, Map2D}, shared::Color};

pub struct TowerMap {
  grid: Vec<Vec<char>>,
  pub anti_node_map: HashSet<(usize, usize)>,
}
impl TowerMap {
  pub fn new(input: &String) -> TowerMap {
    TowerMap {
      grid: TowerMap::build_from_char_text(input),
      anti_node_map: HashSet::new(),
    }
  }

  pub fn get_antinodes(first: &(usize, usize), other: &(usize, usize)) -> [Option<(usize, usize)>; 2] {
    let other_0 = other.0 as isize;
    let other_1 = other.1 as isize;
    let first_0 = first.0 as isize;
    let first_1 = first.1 as isize;

    let diff_x = other_0 - first_0;
    let diff_y = other_1 - first_1;
    
    let first_antinode = (first_0 - diff_x, first_1 - diff_y);
    let second_antinode = (other_0 + diff_x, other_1 + diff_y);

    [
      if first_antinode.0 < 0 || first_antinode.1 < 0 { None } else { Some((first_antinode.0 as usize, first_antinode.1 as usize)) },
      if second_antinode.0 < 0 || second_antinode.1 < 0 { None } else { Some((second_antinode.0 as usize, second_antinode.1 as usize)) },
    ]
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

    for (_lett, pos_list) in node_map.iter() {
      for x in 0..pos_list.len() {
        for y in 1 + x..pos_list.len() {
          let first_pos = pos_list.get(x).expect("cant break");
          let second_pos = pos_list.get(y).expect("cant break");

          for antinode_opt in TowerMap::get_antinodes(&first_pos, &second_pos) {
            if let Some(antinode) = antinode_opt {
              if self.get(&antinode).is_some() {
                self.anti_node_map.insert(antinode);
              }
            }
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
  fn matrix_to_paint(&self) -> &Vec<Vec<impl std::fmt::Display>> {
    &self.grid
  }

  fn color_override(&self, value: impl std::fmt::Display, pos: &(usize, usize)) -> crate::shared::Color<impl std::fmt::Display> {
    if self.anti_node_map.contains(pos) {
      return Color::Blue(value);
    }

    Color::Default(value)
  }
}