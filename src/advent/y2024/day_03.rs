use regex::Regex;
use crate::shared::{print_solution, print_test, read_input, Color};

fn extract_commands(command_string: &str) -> Vec<(i32, i32)> {
  let re = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)").expect("I can't write regex apparently");

  re.captures_iter(command_string).map(|c| {
    let (_match, [_command, a, b]): (&str, [&str; 3]) = c.extract();
    (a.parse().expect("matched something wrong"), b.parse().expect("matched something wrong"))
  }).collect()
}

fn part_one(string: &String) {
  let commands = extract_commands(string);
  let sum: i32 = commands.iter().map(|(a, b)| a * b).sum();
  println!("Sum of commands is: {}", Color::Blue(sum));
}

fn part_two(string: &String) {
  let mut command_str = string.as_str();
  let mut delimiter = "don't()";
  let mut valid_commands: Vec<(i32, i32)> = Vec::new();

  while command_str != "" {
    let (left, right) = command_str.split_once(delimiter).unwrap_or((command_str, ""));
    command_str = right;

    if delimiter == "don't()" /*left is a do (before the don't breaks it)*/ {
      let mut commands = extract_commands(left);
      valid_commands.append(&mut commands);
    }

    delimiter = if delimiter == "don't()" { "do()" } else { "don't()" };
  }

  let sum: i32 = valid_commands.iter().map(|(a, b)| a * b).sum();
  println!("Sum of valid (do()) commands is: {}", Color::Blue(sum));
}

pub fn run() {
  print_test();
  let test_string = read_input("day_03/test");
  part_one(&test_string);
  let test2_string = read_input("day_03/test2");
  part_two(&test2_string);
  println!();
  
  print_solution();
  let string = read_input("day_03/input");
  part_one(&string);
  part_two(&string);
}
