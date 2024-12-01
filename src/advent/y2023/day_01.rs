mod split_line;

use split_line::split_line;

use crate::shared::{read_input, print_test, print_solution, Color};

fn get_first_last_num(numbers: (char, char)) -> u32 {
  let first_last_num = format!("{}{}", numbers.0, numbers.1).parse::<u32>().expect("something went wrong when transforming to num");

  first_last_num
}

fn part_one(file: &str) -> u32 {
  let read_file = read_input(file);
  let lines = read_file.lines();

  let unparsed_numbers = lines.map(|line| split_line(line, false));
  let numbers = unparsed_numbers.map(|n| { get_first_last_num(n) });

  numbers.sum()
}

fn part_two(file: &str) -> u32 {
  let read_file = read_input(file);
  let lines = read_file.lines();

  let unparsed_numbers = lines.map(|line| split_line(line, true));
  let numbers = unparsed_numbers.map(|n| { get_first_last_num(n) });

  numbers.sum()
}

pub fn run() {
  print_test();
  let test_sum_1 = part_one("day_01/test");
  let test_sum_2 = part_two("day_01/test_2");
  println!("P1: test input sum is: {}", Color::Red(test_sum_1));
  println!("P2: test input sum is: {}\n", Color::Red(test_sum_2));
  
  print_solution();
  let real_sum_1 = part_one("day_01/input");
  let real_sum_2 = part_two("day_01/input");
  println!("P1 sum is: {}", Color::Red(real_sum_1));
  println!("P2 sum is: {}\n", Color::Red(real_sum_2));
}
