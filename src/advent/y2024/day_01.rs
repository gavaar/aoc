mod list_holder;

use crate::shared::{print_solution, print_test, read_input, Color};
use list_holder::ListHolder;

fn part_one(uri: &str) {
  let input = read_input(uri);
  let list_holder = ListHolder::new(&input);
  
  let diff_vectors = list_holder.get_diffs_vector();
  println!("Sum of differences is {}", Color::Green(diff_vectors.iter().sum::<u32>()));
}

pub fn run() {
  print_test();
  part_one("day_01/test");

  print_solution();
  part_one("day_01/input");
}
