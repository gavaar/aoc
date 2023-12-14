use std::collections::HashSet;

use crate::shared::{print_test, print_solution, read_input, Color, report_progress};

#[derive(Debug, Hash)]
enum Mirror {
  Vert(usize),
  Hori(usize),
}
impl Mirror {
  pub fn value(&self) -> u32 {
    match self {
      Self::Vert(pos) => (pos * 100) as u32,
      Self::Hori(pos) => *pos as u32,
    }
  }

  pub fn name(&self) -> &str {
    match self {
      Self::Vert(_) => "vertical",
      Self::Hori(_) => "horizontal",
    }
  }
}
impl PartialEq for Mirror {
  fn eq(&self, other: &Self) -> bool {
    self.name() == other.name() && self.value() == other.value()
  }
}
impl Eq for Mirror {}

struct Pattern {
  mirror_map: Vec<Vec<char>>,
}
impl Pattern {
  pub fn new(chart: &String) -> Pattern {
    let mirror_map = chart.lines().map(|line| {
      line.chars().collect()
    }).collect();

    Pattern {
      mirror_map,
    }
  }
}

fn make_both_str_equal_len<'a, 'b>(left: &'a Vec<char>, right: &'b Vec<char>) -> (&'a [char], &'b [char]) {
  let lengths = [left.len(), right.len()];
  let smallest_len = lengths.iter().min().unwrap();
  let left_start = left.len() - smallest_len;
  
  (left.split_at(left_start).1, right.get(..*smallest_len).unwrap())
}

// ---
fn are_bodies_mirrored_vertically(mirror:&Vec<Vec<char>>, pos: usize) -> bool {
  let mut pos_a = pos - 1;
  let mut pos_b = pos;

  loop {
    let line_a = &mirror[pos_a];
    let line_b = &mirror[pos_b];

    if line_a != line_b {
      return false;
    }

    if pos_a == 0 || pos_b == mirror.len() - 1 {
      return true;
    }

    pos_a -= 1;
    pos_b += 1;
  }
}

// ><
fn are_bodies_mirrored_horizontally(mirror: &Vec<Vec<char>>, pos: usize) -> bool {
  for line in mirror {
    let splitted_line = line.split_at(pos);
    let left_vec = splitted_line.0.to_vec();
    let right_vec = splitted_line.1.to_vec();
    let (left, right) = make_both_str_equal_len(&left_vec, &right_vec);

    let left_parsed: Vec<&char> = left.iter().collect();
    let right_reordered: Vec<&char> = right.iter().rev().collect();

    if left_parsed != right_reordered {
      return false;
    }
  }

  return true;
}

fn are_lines_mirrored(line_chars: &Vec<char>, pos: usize) -> bool {
  let mut pos_a = pos - 1;
  let mut pos_b = pos;

  loop {
    let char_a = line_chars.get(pos_a);
    let char_b = line_chars.get(pos_b);

    if char_a != char_b {
      return false;
    }

    // if they are equal, and they are the last ones
    if pos_a == 0 || pos_b == line_chars.len() - 1 {
      return true;
    }

    pos_a -= 1;
    pos_b += 1;
  }
}

fn find_equality_position(mirror_len: usize, chars_vec: &Vec<char>, mirror: &Vec<Vec<char>>, horiz: bool) -> Vec<usize> {
  let mut possible_solutions = Vec::new();

  for pos in 1..mirror_len {
    let previous_value = chars_vec[pos - 1];
    let current_value = chars_vec[pos];
    
    if previous_value == current_value {
      if are_lines_mirrored(&chars_vec, pos) {
        if horiz {
          if are_bodies_mirrored_horizontally(mirror, pos) {
            possible_solutions.push(pos);
          }
        } else {
          if are_bodies_mirrored_vertically(mirror, pos) {
            possible_solutions.push(pos);
          }
        }
      }
    }
  }

  possible_solutions
}

fn search_for_equals(mirror: &Vec<Vec<char>>) -> Vec<Mirror> {
  let first_line = mirror.first().unwrap();
  let mirror_cols = first_line.len();

  let horiz_solutions = find_equality_position(mirror_cols, &first_line, mirror, true);

  let mirror_rows = mirror.len();
  let row_chars: Vec<char> = mirror.iter().map(|line| *line.first().unwrap()).collect();
  let vertical_solutions = find_equality_position(mirror_rows, &row_chars, mirror, false);

  let mut horiz_mirrors: Vec<Mirror> = horiz_solutions.iter().map(|s| Mirror::Hori(*s)).collect();
  let mut vertical_mirrors: Vec<Mirror> = vertical_solutions.iter().map(|s| Mirror::Vert(*s)).collect();

  horiz_mirrors.append(&mut vertical_mirrors);
  horiz_mirrors
}

fn search_for_smudge(pattern: &Pattern) -> Mirror {
  let mut possible_solutions = HashSet::new();
  let mirror_clone = pattern.mirror_map.clone();
  let original_solution = search_for_equals(&mirror_clone);
  let mut pattern_map = pattern.mirror_map.clone();
  let mut old_pos_value = (0, 0, pattern_map[0][0]);

  for row in 0..pattern_map.len() {
    for col in 0..pattern_map[0].len() {
      // if possible_solutions.len() > 1 {
      //   break;
      // }

      // revert change of previous iteration
      pattern_map[old_pos_value.0][old_pos_value.1] = old_pos_value.2;

      // do change in current iteration
      let current_at_pos = pattern_map[row][col];
      old_pos_value = (row, col, current_at_pos);

      let new_at_pos = if current_at_pos == '#' { '.' } else { '#' };
      pattern_map[row][col] = new_at_pos;

      let solutions = search_for_equals(&pattern_map);
      for sol in solutions {
        possible_solutions.insert(sol);
      }
    }
  }

  possible_solutions.remove(&original_solution[0]);
  possible_solutions.into_iter().next().unwrap()
}

fn mirrors(all_mirrors: &String) -> Vec<Pattern> {
  all_mirrors.split("\n\n").map(|m| {
    Pattern::new(&m.to_string())
  }).collect()
}

fn part_one(all_mirrors: &String) {
  let mirrors = mirrors(all_mirrors);
  let solution = mirrors.iter().map(|mirror| {
    let solutions = search_for_equals(&mirror.mirror_map);
    solutions[0].value()
  }).sum::<u32>();

  println!("mirror sum: {}", Color::Red(solution));
}

fn part_two(all_mirrors: &String) {
  let patterns = mirrors(all_mirrors);
  let total = patterns.len();
  let mut smudged_solutions = Vec::new();

  for (curr, mirror) in patterns.iter().enumerate() {
    report_progress(curr, total);

    let solutions = search_for_smudge(&mirror);
    smudged_solutions.push(solutions.value());
  }

  println!("mirror sum: {}", Color::Red(smudged_solutions.iter().sum::<u32>()));
}

pub fn run() {
  print_test();
  let mirrors_test = read_input("day_13/test");
  println!("-- p1");
  part_one(&mirrors_test);
  println!("-- p2");
  part_two(&mirrors_test);

  println!();
  
  print_solution();
  let mirrors = read_input("day_13/input");
  println!("-- p1");
  part_one(&mirrors);
  println!("-- p2");
  part_two(&mirrors);
}
