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

  pub fn joltage_overriden(&self) -> u128 {
    let mut found = vec![];
    let mut next_start = 0;

    for idx in (0..12).rev() {
      let battery_slice = &self.batteries[next_start..&self.batteries.len() - idx];
      let (nex_start_idx, biggest_num) = find_biggest_possible_at_index(battery_slice);
      next_start += nex_start_idx + 1;
      found.push(biggest_num);
    }

    found.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("").parse::<u128>().expect("found list did not make a number")
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

  let overriden_joltage = test_batteries.iter().enumerate().map(|(r_idx, bat)| {
    report_progress(r_idx, total);
    return bat.joltage_overriden();
  }).sum::<u128>();
  println!("Total OVERLOADED joltage: {}", Color::Red(overriden_joltage));

  println!();
  
  print_solution();
  let batteries = build_batteries("day_03/input");
  let total = batteries.len();

  let total_output_joltage = batteries.iter().enumerate().map(|(r_idx, bat)| {
    report_progress(r_idx, total);
    return bat.joltage() as u128;
  }).sum::<u128>();
  println!("Total joltage: {}", Color::Yellow(total_output_joltage));

  let overriden_joltage = batteries.iter().enumerate().map(|(r_idx, bat)| {
    report_progress(r_idx, total);
    return bat.joltage_overriden();
  }).sum::<u128>();
  println!("Total OVERLOADED joltage: {}", Color::Red(overriden_joltage));
}
