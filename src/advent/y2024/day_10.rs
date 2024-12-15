mod hike_map;

use crate::shared::{print_solution, print_test, read_input, Color};
use hike_map::HikeMap;

fn part_both(input: &String) {
  let mut map = HikeMap::new(&input);
  map.find_trailheads();
  let sum: u32 = map.trailheads.values().map(|v| v.iter().count() as u32).sum();
  let rating: u32 = map.trailheads.values().map(|v| v.values().sum::<u32>()).sum();
  println!("The trailheads score is {}", Color::Blue(sum));
  println!("The trailhead rating is {}", Color::Blue(rating));
}

pub fn run() {
  print_test();
  let test_str = read_input("day_10/test");
  part_both(&test_str);
  println!();

  print_solution();
  let str = read_input("day_10/input");
  part_both(&str);
}
