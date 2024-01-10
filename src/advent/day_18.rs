use std::collections::HashMap;

use crate::{shared::{print_test, read_input, Color, print_solution}, advent::day_18::helpers::draw};

mod pos;
mod dir;
mod instr;
mod helpers;
use helpers::build_perimeter;

use self::helpers::{angle, instructions_and_starting_pos};

fn count_gaps(row_map: &mut HashMap<usize, Vec<usize>>) -> u32 {
  let mut idx = 0usize;
  let mut sum = 0u32;
  let init_len = row_map.len();

  while idx < init_len {
    let mut cols = row_map.get(&idx).expect("extract from map should not fail").clone();
    cols.sort();
    let mut cols_iter = cols.iter();

    let mut prev = *cols_iter.next().expect("should not fail");
    let mut prev_angle = angle(&row_map, idx, prev);

    sum += 1; // col I just removed

    let mut inside = false;

    cols_iter.for_each(|current_col| {
      if current_col - prev > 1 {
        // if this >[#]### has a different angle than ###[#]< this, or
        // it's a vertical line, we swap inside value
        let new_prev_angle = angle(&row_map, idx, prev);
        if prev_angle == "both" || new_prev_angle != prev_angle {
          inside = !inside;
        }
        
        // then we set the new prev_angle to be the new left corner
        prev_angle = angle(&row_map, idx, *current_col);

        if inside {
          // only sum the ones inside
          sum += (current_col - prev) as u32 - 1;
        }
      }

      sum += 1; // sum current col
      prev = *current_col;
    });

    idx += 1;
  }

  sum
}

fn dig_hole(input: &String) -> HashMap<usize, Vec<usize>> {
  let (instr, starting) = instructions_and_starting_pos(input);
  build_perimeter(instr, starting)
}

pub fn run() {
  print_test();
  let test = read_input("day_18/test");
  let mut test_map = dig_hole(&test);
  draw(&test_map);
  let test_sum = count_gaps(&mut test_map);

  println!("sum: {}", Color::Red(test_sum));
  println!();

  print_solution();
  let input = read_input("day_18/input");
  let mut map = dig_hole(&input);
  // draw(&map);
  let sum = count_gaps(&mut map);
  
  println!("sum: {}", Color::Red(sum));
}
