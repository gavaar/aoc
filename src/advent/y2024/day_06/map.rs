use std::collections::{HashMap, HashSet};

use crate::shared::report_progress;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Direction {
  North,
  East,
  South,
  West,
}
impl Direction {
  pub fn next(&self, curr_pos: &(usize, usize)) -> Option<(usize, usize)> {
    match self {
      Direction::North => {
        if curr_pos.1 == 0 {
          return None;
        }
        return Some((curr_pos.0, curr_pos.1 - 1));
      }
      Direction::East => {
        return Some((curr_pos.0 + 1, curr_pos.1))
      }
      Direction::South => {
        return Some((curr_pos.0, curr_pos.1 + 1));
      }
      Direction::West => {
        if curr_pos.0 == 0 {
          return None;
        }
        return Some((curr_pos.0 - 1, curr_pos.1));
      }
    }
  }
}

pub struct Cop<'a> {
  facing: Direction,
  pos: (usize, usize),
  history: HashSet<(usize, usize)>,
  path: HashSet<((usize, usize), Direction)>,
  would_obstruct: HashSet<(usize, usize)>,
  left_maze: bool,
  forbidden_pos: (usize, usize),
  map: &'a Map,
}
impl<'a> Cop<'a> {
  pub fn new(map: &Map) -> Cop {
    let pos = map.starting_cop_pos;
    let history = HashSet::new();
    let path = HashSet::new();
    let would_obstruct = HashSet::new();
    let map = map;
    let facing = Direction::North;
    let forbidden_pos = (9999, 9999);
    let left_maze = false;

    Cop { pos, history, path, map, facing, forbidden_pos, left_maze, would_obstruct }
  }

  pub fn would_obstruct(&self) -> usize {
    self.would_obstruct.len()
  }

  pub fn warp(&mut self, pos: (usize, usize)) {
    self.pos = pos;
  }

  pub fn look_towards(&mut self, dir: &Direction) {
    self.facing = dir.clone();
  }

  pub fn forbid_pos(&mut self, pos: (usize, usize)) {
    self.forbidden_pos = pos;
  }

  pub fn visited(&self) -> usize {
    self.history.len()
  }

  pub fn walk(&mut self, is_clone: bool) {
    loop {
      if !is_clone {
        report_progress(self.history.len(), self.map.grid.len());
      };
      self.history.insert(self.pos);
      let cop_is_in_loop = !self.path.insert((self.pos, self.facing.clone()));
      if cop_is_in_loop { return; };

      let Some(frontal_pos) = self.facing.next(&self.pos) else {
        self.left_maze = true;
        return;
      };
      let Some(frontal_cell) = self.map.grid_value(&frontal_pos) else {
        self.left_maze = true;
        return;
      };
      
      if frontal_cell == &'#' || frontal_pos == self.forbidden_pos {
        self.rotate();
        continue;
      }

      if !is_clone && frontal_pos != self.map.starting_cop_pos && self.history.get(&frontal_pos).is_none() {
        let mut cop_clone = Cop::new(&self.map);
        cop_clone.warp(self.pos);
        cop_clone.forbid_pos(frontal_pos);
        cop_clone.look_towards(&self.facing);
        cop_clone.walk(true);
        if !cop_clone.left_maze {
          self.would_obstruct.insert(frontal_pos);
        }
      }

      self.pos = frontal_pos;
    }
  }

  fn rotate(&mut self) {
    self.facing = match self.facing {
      Direction::North => Direction::East,
      Direction::East => Direction::South,
      Direction::South => Direction::West,
      Direction::West => Direction::North,
    }
  }
}

pub struct Map {
  grid: HashMap<(usize, usize), char>,
  starting_cop_pos: (usize, usize),
}
impl Map {
  pub fn new(input: &String) -> Map {
    let mut grid = HashMap::new();
    let mut starting_cop_pos= (0, 0);

    for (y, line) in input.lines().enumerate() {
      for (x, char) in line.chars().enumerate() {
        if char == '^' {
          starting_cop_pos = (x, y);
          grid.insert((x, y), '.');
        } else {
          grid.insert((x, y), char);
        }
      }
    }

    Map { grid, starting_cop_pos }
  }

  pub fn grid_value(&self, pos: &(usize, usize)) -> Option<&char> {
    self.grid.get(pos)
  }
}
