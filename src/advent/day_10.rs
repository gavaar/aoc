use crate::shared::{print_test, print_solution, read_input};

// top, right, down, left
fn pipe_connects(pipe: char) -> [bool; 4] {
  match pipe {
    '|' => [true, false, true, false],
    '-' => [false, true, false, true],
    'L' => [true, true, false, false],
    'J' => [true, false, false, true],
    '7' => [false, false, true, true],
    'F' => [false, true, true, false],
    'S' => [true, true, true, true],
    _ => [false, false, false, false],
  }
}

#[derive(Debug)]
struct Map {
  map: Vec<Vec<char>>,
  starting_pos: (usize, usize),
}
impl Map {
  pub fn new(chart: &String) -> Map {
    let mut map: Vec<Vec<char>> = Vec::with_capacity(chart.lines().count());
    let mut starting_pos: (usize, usize) = (0, 0);

    for (row, line) in chart.lines().enumerate() { // O(N^2)
      map.push(Vec::new());
      for (col, char) in line.chars().enumerate() {
        if char == 'S' {
          starting_pos = (row, col);
        }
        map[row].push(char);
      }
    }

    Map { map, starting_pos }
  }

  fn pipe_in_pos(&self, origin: &(usize, usize)) -> char {
    self.map[origin.0][origin.1]
  }

  // pos.x pos.y pos.dir
  pub fn find_next(&self, origin: (usize, usize), skip: usize) -> (usize, usize, usize) {
    let pipe_connects = pipe_connects(self.pipe_in_pos(&origin));

    let permitted = pipe_connects.iter().enumerate()
      .filter_map(|(i, connects)| {
        if *connects {
          return Some(i);
        }

        None
      });

    for i in permitted {
      if skip == i {
        continue;
      }

      let sol = match i {
        0 => self.top(origin),
        1 => self.right(origin),
        2 => self.bot(origin),
        _ => self.left(origin),
      };

      let from = match i {
        0 => 2,
        1 => 3,
        2 => 0,
        _ => 1,
      };

      if let Some(target_pos) = sol {
        return (target_pos.0, target_pos.1, from);
      }
    }

    unreachable!();
  }

  pub fn top(&self, origin: (usize, usize)) -> Option<(usize, usize)> {
    if origin.0 == 0 { return None; }
    self.target((origin.0 - 1, origin.1), 2)
  }

  pub fn bot(&self, origin: (usize, usize)) -> Option<(usize, usize)> {
    if origin.0 + 1 == self.map.len() { return None; }
    self.target((origin.0 + 1, origin.1), 0)
  }

  pub fn left(&self, origin: (usize, usize)) -> Option<(usize, usize)> {
    if origin.1 == 0 { return None; }
    self.target((origin.0, origin.1 - 1), 1)
  }

  pub fn right(&self, origin: (usize, usize)) -> Option<(usize, usize)> {
    if origin.1 + 1 == self.map[0].len() { return None; }
    self.target((origin.0, origin.1 + 1), 3)
  }
  
  fn target(&self, target: (usize, usize), connects_to: usize) -> Option<(usize, usize)> {
    let pipe = self.map[target.0][target.1];
    if pipe == '.' {
      return None;
    }

    if pipe_connects(pipe)[connects_to] {
      return Some(target);
    }

    None
  }
}

fn part_one(chart: &String) {
  let map = Map::new(chart);
  let mut current_pos = map.starting_pos.clone();
  let mut came_from = 9;
  let mut steps: Vec<(usize, usize)> = Vec::new();

  loop {
    let (row, col, from) = map.find_next(current_pos, came_from);
    came_from = from;
    current_pos = (row, col);

    if current_pos == map.starting_pos {
      break;
    }

    steps.push((row, col));
  }

  println!("final: {:?}\nfarthest: {}", steps.last(), (steps.len() + 1) / 2);
}

pub fn run() {
  print_test();
  let small_chart = read_input("day_10/test_1");
  part_one(&small_chart);
  println!();

  let medium_chart = read_input("day_10/test_2");
  part_one(&medium_chart);
  println!();
  
  print_solution();
  let chart = read_input("day_10/input");
  part_one(&chart);
}
