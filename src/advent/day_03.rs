use std::fs;

mod map;
use map::Map;

fn read_input(location: &str) -> Map {
  let input = fs::read_to_string(location).expect("error reading file");
  Map::new(input)
}

fn part_one() {
  println!("#### TEST ####");
  let input_map = read_input("src/advent/day_03/test");
  let valid_nums = input_map.valid_numbers();
  println!("the sum is: {}\n", valid_nums.iter().sum::<u64>());
  
  println!("#### SOLUTION ####");
  let input_map = read_input("src/advent/day_03/input");
  let valid_nums = input_map.valid_numbers();
  println!("the sum is: {}\n", valid_nums.iter().sum::<u64>());
}

pub fn run() {
  part_one();
}
