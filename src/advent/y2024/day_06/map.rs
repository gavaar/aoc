use std::collections::{HashMap, HashSet};

#[derive(PartialEq)]
enum Direction {
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

pub struct Map {
  last_x: usize,
  last_y: usize,
  grid: HashMap<(usize, usize), char>,
  visited: HashSet<(usize, usize)>,
  cop_pos: (usize, usize),
  cop_facing: Direction,
}
impl Map {
  pub fn new(input: &String) -> Map {
    let last_y = input.lines().count() - 1;
    let last_x = input.lines().next().unwrap().chars().count() - 1;
    let cop_facing = Direction::North;
    let mut grid = HashMap::new();
    let mut cop_pos= (0, 0);
    let mut visited = HashSet::new();

    for (y, line) in input.lines().enumerate() {
      for (x, char) in line.chars().enumerate() {
        if char == '^' {
          cop_pos = (x, y);
          visited.insert((x, y));
          grid.insert((x, y), '.');
        } else {
          grid.insert((x, y), char);
        }
      }
    }

    Map { last_x, last_y, grid, visited, cop_pos, cop_facing }
  }

  pub fn wait_for_cop_to_leave(&mut self) {
    while !self.is_cop_leaving() {
      let next_pos = self.cop_facing.next(&self.cop_pos).expect("bug in code, got pos out of bounds");
      let collission = *self.grid.get(&next_pos).expect("pos should exist") == '#';
      if collission {
        self.rotate_cop();
        continue;
      }

      self.cop_pos = self.cop_facing.next(&self.cop_pos).expect("should return");
      self.visited.insert(self.cop_pos.clone());
    }
  }

  pub fn visited_places(&self) -> usize {
    self.visited.len()
  }

  // private stuff
  fn is_cop_leaving(&self) -> bool {
    match self.cop_facing {
      Direction::North => self.cop_pos.1 == 0,
      Direction::East => self.cop_pos.0 == self.last_x,
      Direction::South => self.cop_pos.1 == self.last_y,
      Direction::West => self.cop_pos.0 == 0,
    }
  }

  fn rotate_cop(&mut self) {
    self.cop_facing = match self.cop_facing {
      Direction::North => Direction::East,
      Direction::East => Direction::South,
      Direction::South => Direction::West,
      Direction::West => Direction::North,
    };
  }
}
