mod map;

use crate::shared::{print_solution, print_test, read_input, Color};
use map::{Cop, Map};

fn build_map(uri: &str) -> Map {
  let input = read_input(uri);
  Map::new(&input)
}

fn part_one(map: &mut Map) {
  let mut cop = Cop::new(map);
  cop.walk(false);
  println!("\nVisited {} places", Color::Blue(cop.visited()));
  println!("Would be obstructed by {} places", Color::Blue(cop.would_obstruct()));
}

pub fn run() {
  print_test();
  let mut test_map = build_map("day_06/test");
  part_one(&mut test_map);
  println!();

  print_solution();
  let mut map = build_map("day_06/input");
  part_one(&mut map);
}