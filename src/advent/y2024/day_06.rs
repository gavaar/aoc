mod map;

use crate::shared::{print_solution, print_test, read_input, Color};
use map::Map;

fn build_map(uri: &str) -> Map {
  let input = read_input(uri);
  Map::new(&input)
}

fn part_one(map: &mut Map) {
  map.wait_for_cop_to_leave();
  let visited_places = map.visited_places();
  println!("Visited {} places", Color::Blue(visited_places))
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