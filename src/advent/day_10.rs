use crate::shared::{print_test, print_solution, read_input, Color};

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

  pub fn starting_pipe(&self) -> char {
    let top = self.top(self.starting_pos);
    let right = self.right(self.starting_pos);
    let bot = self.bot(self.starting_pos);
    let left = self.left(self.starting_pos);

    match (top.is_some(), right.is_some(), bot.is_some(), left.is_some()) {
      (true, true, false, false) => 'L',
      (true, false, true, false) => '|',
      (true, false, false, true) => 'J',
      (false, true, true, false) => 'F',
      (false, true, false, true) => '-',
      (false, false, true, true) => '7',
      _ => unreachable!()
    }
  }

  pub fn build_road(&self) -> Vec<(usize, usize)> {
    let mut current_pos = self.starting_pos.clone();
    let mut came_from = 9;
    let mut steps: Vec<(usize, usize)> = Vec::new();

    loop {
      let (row, col, from) = self.find_next(current_pos, came_from);
      came_from = from;
      current_pos = (row, col);

      if current_pos == self.starting_pos {
        break;
      }

      steps.push((row, col));
    }

    steps
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
  let steps = map.build_road();

  println!("farthest: {}", Color::Red((steps.len() + 1) / 2));
}

fn part_two(chart: &String) {
  let map = Map::new(chart);
  let steps = map.build_road();
  let mut burrow_size = 0;

  for (row, line) in chart.lines().enumerate() {
    let mut inside = false;
    let mut expect = 'X';

    for (col, dirty_char) in line.chars().enumerate() {
      let char = if dirty_char == 'S' {
        map.starting_pipe()
      } else {
        dirty_char
      };

      let is_in_map = map.starting_pos == (row, col) || steps.iter().find(|v| *v == &(row, col)).is_some();

      if is_in_map {
        if char == '|' {
          inside = !inside;
        }

        if char == 'F' {
          expect = 'J';
        }

        if char == 'L' {
          expect = '7';
        }

        if char == 'J' && expect == 'J' {
          inside = !inside;
        }

        if char == '7' && expect == '7' {
          inside = !inside;
        }

        if char != '-' && char != 'F' && char != 'L' {
          expect = 'X';
        }
      }

      if is_in_map {
        print!("{char}");
      } else if inside {
        burrow_size += 1;
        print!("{}", Color::Green(char))
      } else {
        print!("{}", Color::Red(char))
      }
    }

    println!();
  }

  println!("the size is {burrow_size}");
}

pub fn run() {
  print_test();
  let small_chart = read_input("day_10/test_1");
  println!("small chart");
  part_one(&small_chart);
  println!();
  
  println!("medium chart");
  let medium_chart = read_input("day_10/test_2");
  part_one(&medium_chart);
  println!();

  println!("small enclose");
  let small_enclose =read_input("day_10/test_3");
  part_two(&small_enclose);
  println!();

  println!("medium enclose");
  let mid_enclose =read_input("day_10/test_4");
  part_two(&mid_enclose);
  println!();

  println!("large enclose");
  let large_enclose =read_input("day_10/test_5");
  part_two(&large_enclose);
  println!();

  print_solution();
  let chart = read_input("day_10/input");
  part_one(&chart);

  let enclose = read_input("day_10/input");
  part_two(&enclose);
}
