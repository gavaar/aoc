use crate::{shared::{read_input, print_test, print_solution, Color}, advent::y2023::day_06::helpers::number_of_ways_to_break_record};

mod helpers;
mod race;
pub use race::Race;

fn part_one(input: &str) {
  let input = read_input(input);
  let races = helpers::parse_input(input);
  let Some(num_of_ways) = helpers::number_of_ways_to_break_record(races).into_iter().reduce(|acc, e| acc * e) else {
    return println!("something broke when multiplying the amount of solutions for each race");
  };
  println!("num of ways multiplier is {}", Color::Red(num_of_ways));
}

fn part_two(input: &str) {
  let input = read_input(input);
  let clean = helpers::clean_input_whitespaces(input);
  let races = helpers::parse_input(clean);

  let Some(num_of_ways) = number_of_ways_to_break_record(races).into_iter().reduce(|acc, e| acc * e) else {
    return println!("something broke when multiplying the amount of solutions for each race");
  };

  println!("clean multiplier is {}\n", Color::Red(num_of_ways));
}

pub fn run() {
  print_test();
  part_one("day_06/test");
  part_two("day_06/test");
  
  print_solution();
  part_one("day_06/input");
  part_two("day_06/input");
}
