use std::collections::HashMap;

use crate::shared::{print_test, print_solution, read_input, Color, report_progress};

fn matches(expected: &Vec<String>, decoded: &str, leftover: usize, memory: &mut HashMap<String, u128>) -> u128 {
  let mut original_expected = expected.iter();
  let original_expected_key = expected.join(",");
  let expected_value = original_expected.next().unwrap();
  let (decoded_sub, leftover_sub) = decoded.split_at(expected_value.len());

  let memory_key = format!("{}::{}", original_expected_key, decoded);

  if let Some(already_parsed) = memory.get(&memory_key) {
    return *already_parsed;
  }

  let mut decoded_chars = decoded_sub.chars();
  let mut expected_chars = expected_value.chars();

  while let Some(next_matching_char) = expected_chars.next() {
    let decoded_char = decoded_chars.next();
    
    if let Some(char_value) = decoded_char {
      if char_value == '?' {
        continue;
      }

      if char_value != next_matching_char {
        return 0;
      }
    }
  }

  let sub_expected: Vec<String> = original_expected.map(|v| v.to_owned()).collect();
  let sub_decoded = String::from_iter(leftover_sub.chars());

  if sub_expected.len() == 0 {
    if sub_decoded.contains('#') {
      return 0;
    }

    return 1;
  }

  let mut sum = 0;

  for start_point in 0..leftover + 1 {
    let (prev, split_decoded) = sub_decoded.split_at(start_point);

    if prev.contains('#') {
      continue;
    }

    let matches = matches(&sub_expected, split_decoded, leftover - start_point, memory);
    sum += matches;
  }

  memory.insert(memory_key, sum);
  
  sum
}

fn valid_outputs(expected: Vec<usize>, decoded: &str, memory: &mut HashMap<String, u128>) -> u128 {
  let min_solution: Vec<String> = expected.iter().map(|exp_len| {
    "#".repeat(*exp_len) + "."
  }).collect();

  let available_spaces = decoded.len() - (min_solution.iter().map(|v| v.len()).sum::<usize>() - 1);

  let mut matching_possibilities = 0;

  for start_point in 0..available_spaces + 1 {
    let (prev, split_decoded) = decoded.split_at(start_point);

    if prev.contains('#') {
      continue; 
    }

    let matches = matches(&min_solution, format!("{}.", split_decoded).as_str(), available_spaces - start_point, memory);
    matching_possibilities += matches;
  }

  matching_possibilities
}

fn part_one(input: &String) {
  let lines = input.lines();
  let mut mem = HashMap::new();

  let sum = lines.map(|line| {
    let (decode, expected_str) = line.split_once(' ').expect("broke when splittin");
    let expected = expected_str.split(',').map(|v| v.parse::<usize>().expect("expected value broke when usize")).collect::<Vec<usize>>();

    valid_outputs(expected, decode, &mut mem)
  }).sum::<u128>();

  println!("sum: {}", Color::Red(sum));
}

fn part_two(input: &String) {
  let lines = input.lines();
  let mut memory = HashMap::new();

  let sum = lines.enumerate().map(|(current, line)| {
    print!(" @ {current}");
    report_progress(current, input.lines().count());

    let (decode, expected_str) = line.split_once(' ').expect("broke when splittin");
    let expected = expected_str.split(',').map(|v| v.parse::<usize>().expect("expected value broke when usize")).collect::<Vec<usize>>();

    let mut result = 0;

    for repeat in 1..6 {
      let mut extended_decode = format!("{}?", decode).repeat(repeat);
      let extended_expected = expected.repeat(repeat);
      extended_decode.pop();

      result = valid_outputs(extended_expected, extended_decode.as_str(), &mut memory);
    }

    result
  }).sum::<u128>();

  println!("\nsum: {}", Color::Red(sum));
}

pub fn run() {
  print_test();
  let test = read_input("day_12/test");
  println!("-- p1");
  part_one(&test);
  println!("-- p2");
  part_two(&test);

  println!();
  
  print_solution();
  let input = read_input("day_12/input");
  println!("-- p1");
  part_one(&input);
  println!("-- p2");
  part_two(&input);
}
