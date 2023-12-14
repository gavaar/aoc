use crate::shared::{print_test, print_solution, read_input, Color};

#[derive(Debug)]
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
}

fn make_both_str_equal_len<'a, 'b>(left: &'a str, right: &'b str) -> (&'a str, &'b str) {
  let lengths = [left.len(), right.len()];
  let smallest_len = lengths.iter().min().unwrap();
  let left_start = left.len() - smallest_len;
  
  (left.get(left_start..).unwrap(), right.get(..*smallest_len).unwrap())
}

// ---
fn are_bodies_mirrored_vertically(mirror: &str, pos: usize) -> bool {
  let lines_vec: Vec<&str> = mirror.lines().collect();
  let mut pos_a = pos - 1;
  let mut pos_b = pos;

  loop {
    let line_a = lines_vec.get(pos_a);
    let line_b = lines_vec.get(pos_b);

    if line_a != line_b {
      return false;
    }

    if pos_a == 0 || pos_b == lines_vec.len() - 1 {
      return true;
    }

    pos_a -= 1;
    pos_b += 1;
  }
}

// ><
fn are_bodies_mirrored_horizontally(mirror: &str, pos: usize) -> bool {
  for line in mirror.lines() {
    let (left_long, right_long) = line.split_at(pos);
    let (left, right) = make_both_str_equal_len(left_long, right_long);

    let right_reordered: String = right.chars().rev().collect();

    if left != right_reordered.as_str() {
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

fn find_equality_position(mirror_len: usize, chars_vec: &Vec<char>, mirror: &str, horiz: bool) -> Option<usize> {
  for pos in 1..mirror_len {
    let previous_value = chars_vec[pos - 1];
    let current_value = chars_vec[pos];
    
    if previous_value == current_value {
      if are_lines_mirrored(&chars_vec, pos) {
        if horiz {
          if are_bodies_mirrored_horizontally(mirror, pos) {
            return Some(pos);
          }
        } else {
          if are_bodies_mirrored_vertically(mirror, pos) {
            return Some(pos);
          }
        }
      }
    }
  }

  None
}

fn search_for_equals(mirror: &str) -> Mirror {
  let first_line = mirror.lines().next().unwrap();
  let mirror_cols = first_line.len();
  let line_chars: Vec<char> = first_line.chars().collect();

  if let Some(column_pos) = find_equality_position(mirror_cols, &line_chars, mirror, true) {
    return Mirror::Hori(column_pos);
  }

  let mirror_rows = mirror.lines().count();
  let row_chars: Vec<char> = mirror.lines().map(|l| l.chars().next().unwrap()).collect();
  if let Some(row_pos) = find_equality_position(mirror_rows, &row_chars, mirror, false) {
    return Mirror::Vert(row_pos);
  }

  println!("Error!! no match on vertical or horizontal!");
  unreachable!()
}

fn mirrors(all_mirrors: &String) -> Vec<&str> {
  all_mirrors.split("\n\n").collect()
}

fn part_one(all_mirrors: &String) {
  let mirrors = mirrors(all_mirrors);

  let solution: Vec<Mirror> = mirrors.into_iter().map(|mirror| search_for_equals(mirror)).collect();

  println!("mirror sum: {}", Color::Red(solution.iter().map(|m| m.value()).sum::<u32>()));
}

pub fn run() {
  print_test();
  let mirrors_test = read_input("day_13/test");
  part_one(&mirrors_test);

  println!();

  print_solution();
  let mirrors = read_input("day_13/input");
  part_one(&mirrors);
}
