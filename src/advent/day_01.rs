mod split_line;
use std::fs;

use split_line::split_line;

fn get_first_last_num(numbers: (char, char)) -> u32 {
  let first_last_num = format!("{}{}", numbers.0, numbers.1).parse::<u32>().expect("something went wrong when transforming to num");

  println!("{first_last_num}");
  first_last_num
}

fn part_one(file: &str) -> u32 {
  let read_file = fs::read_to_string(file).expect("error reading file");
  let lines = read_file.lines();

  let unparsed_numbers = lines.map(|line| split_line(line, false));
  let numbers = unparsed_numbers.map(|n| { get_first_last_num(n) });

  numbers.sum()
}

fn part_two(file: &str) -> u32 {
  let read_file = fs::read_to_string(file).expect("error reading file");
  let lines = read_file.lines();

  let unparsed_numbers = lines.map(|line| split_line(line, true));
  let numbers = unparsed_numbers.map(|n| { get_first_last_num(n) });

  numbers.sum()
}

pub fn run() {
  println!("\n###  TEST  ###\n");
  let test_sum_1 = part_one("src/advent/day_01/test");
  let test_sum_2 = part_two("src/advent/day_01/test_2");
  println!("P1: test input sum is: {}", test_sum_1);
  println!("P2: test input sum is: {}", test_sum_2);
  
  println!("\n###  SOLUTION  ###\n");
  let real_sum_1 = part_one("src/advent/day_01/input");
  let real_sum_2 = part_two("src/advent/day_01/input");
  println!("P1 sum is: {}", real_sum_1);
  println!("P2 sum is: {}", real_sum_2);
}
