use crate::shared::{Color, print_solution, print_test, read_input, report_progress};

fn find_biggest_possible_at_index(list: &[u8]) -> (usize, &u8) {
  for i in (1..=9u8).rev() {
    if let Some(found) = list.iter().enumerate().find(|(_idx, value)| **value == i) {
      return found;
    }
  }

  unreachable!();
}

struct Bank {
  batteries: Vec<u8>,
}
impl Bank {
  pub fn new(bank: &str) -> Bank {
    let batteries = bank.chars().map(|c| c as u8 - 0x30).collect();
    Bank { batteries }
  }

  pub fn joltage(&self) -> u8 {
    let batteries_without_last = &self.batteries[0..&self.batteries.len() - 1];
    let (break_idx, first_number) = find_biggest_possible_at_index(batteries_without_last);
    
    let second_battery_list = &self.batteries[break_idx + 1..];
    let (_, second_number) = find_biggest_possible_at_index(second_battery_list);

    return format!("{first_number}{second_number}").parse::<u8>().expect("parsed number seems to not be a number");
  }
}

fn build_batteries(uri: &str) -> Vec<Bank> {
  let input = read_input(uri);
  input.lines().map(|line| Bank::new(line)).collect()
}

pub fn run() {
  print_test();
  let test_batteries = build_batteries("day_03/test");
  let total = test_batteries.len();

  let total_output_joltage = test_batteries.iter().enumerate().map(|(r_idx, bat)| {
    report_progress(r_idx, total);
    return bat.joltage() as u128;
  }).sum::<u128>();
  println!("Total joltage: {}", Color::Yellow(total_output_joltage));

  println!();
  
  print_solution();
  let batteries = build_batteries("day_03/input");
  let total = batteries.len();

  let total_output_joltage = batteries.iter().enumerate().map(|(r_idx, bat)| {
    report_progress(r_idx, total);
    return bat.joltage() as u128;
  }).sum::<u128>();
  println!("Total joltage: {}", Color::Yellow(total_output_joltage));
}
