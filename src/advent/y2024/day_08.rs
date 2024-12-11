mod tower_map;

use crate::{components::Drawable, shared::{print_solution, print_test, read_input}};

use tower_map::TowerMap;

fn build_tower_map(uri: &str) -> TowerMap {
  let input = read_input(uri);
  TowerMap::new(&input)
}

fn part_one(tower_map: &mut TowerMap) {
  tower_map.build_anti_nodes();
  tower_map.draw();
  println!("The amount of antinodes is {}", tower_map.anti_node_map.len());
}

fn part_two(tower_map: &mut TowerMap) {
  tower_map.reset();
  tower_map.limit_to_1 = false;
  tower_map.build_anti_nodes();
  tower_map.draw();
  println!("The new amount of antinodes is {}", tower_map.anti_node_map.len());
}

pub fn run() {
  print_test();
  let mut test_tower_map = build_tower_map("day_08/test");
  part_one(&mut test_tower_map);
  let mut test_2_tower_map = build_tower_map("day_08/test_2");
  part_two(&mut test_2_tower_map);
  println!();

  print_solution();
  let mut tower_map = build_tower_map("day_08/input");
  part_one(&mut tower_map);
  part_two(&mut tower_map);
}
