mod map;
use map::Map;

use crate::shared::read_input;

fn read_map(location: &str) -> Map {
  let input = read_input(location);
  Map::new(input)
}

fn part_one() {
  println!("#### TEST ####");
  let input_map = read_map("day_03/test");
  let valid_nums = input_map.valid_numbers();
  println!("the sum is: {}\n", valid_nums.iter().sum::<u64>());
  
  println!("#### SOLUTION ####");
  let input_map = read_map("day_03/input");
  let valid_nums = input_map.valid_numbers();
  println!("the sum is: {}\n", valid_nums.iter().sum::<u64>());
}

fn part_two() {
  println!("#### TEST ####");
  let input_map = read_map("day_03/test");
  let gears = input_map.gears();
  let sum = gears.values().map(|nums| {
      if nums.len() == 2 {
        return nums[0] * nums[1];
      }

      return 0;
    }).sum::<u64>();
  println!("{:#?}", sum);
  
  println!("#### SOLUTION ####");
  let input_map = read_map("day_03/input");
  let gears = input_map.gears();
  let sum = gears.values().map(|nums| {
      if nums.len() == 2 {
        return nums[0] * nums[1];
      }

      return 0;
    }).sum::<u64>();
  println!("{:#?}", sum);
}

pub fn run() {
  part_one();
  part_two();
}
