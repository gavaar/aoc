mod hike_map;

use crate::shared::{print_solution, print_test, read_input, Color};
use hike_map::HikeMap;

fn part_one(input: &String) {
  let mut map = HikeMap::new(&input);
  map.find_trailheads();
  let sum: u32 = map.trailheads.values().map(|v| v.iter().count() as u32).sum();
  println!("The trailheads score is {}", Color::Blue(sum));
}

pub fn run() {
  print_test();
  let test_str = read_input("day_10/test");
  part_one(&test_str);
  println!();

  print_solution();
  let str = read_input("day_10/input");
  part_one(&str);
}
