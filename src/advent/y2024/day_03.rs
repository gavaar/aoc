use regex::Regex;
use crate::shared::{print_solution, print_test, read_input, Color};

fn extract_commands(uri: &str) -> Vec<(i32, i32)> {
  let command_string = read_input(uri);
  let re = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)").expect("I can't write regex apparently");

  re.captures_iter(command_string.as_str()).map(|c| {
    let (_match, [_command, a, b]): (&str, [&str; 3]) = c.extract();
    (a.parse().expect("matched something wrong"), b.parse().expect("matched something wrong"))
  }).collect()
}

fn part_one(commands: &Vec<(i32, i32)>) {
  let sum: i32 = commands.iter().map(|(a, b)| a * b).sum();
  println!("Sum of commands is: {}", Color::Blue(sum));
}

pub fn run() {
  print_test();
  let commands = extract_commands("day_03/test");
  part_one(&commands);
  println!();
  
  print_solution();
  let commands = extract_commands("day_03/input");
  part_one(&commands);
}
