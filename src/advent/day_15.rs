use crate::shared::{print_test, print_solution, read_input, Color, report_progress};

mod boxes;
use boxes::Boxes;

fn parse_input<'a>(input: &'a String) -> Vec<&'a str> {
  input.split(',').collect()
}

fn part_one(input: &String) {
  let sum: u32 = parse_input(input)
  .iter()
  .map(|seq| Boxes::hash_algorithm(seq))
  .sum();

  println!("the crazy ascii sum is {}", Color::Red(sum));
}

fn part_two(input: &String) {
  let sequences = parse_input(input);
  let mut boxes: Boxes = Boxes::default();

  let total = sequences.len();

  sequences.iter().enumerate().for_each(|(curr, seq)| {
    boxes.operate(seq);
    report_progress(curr, total);
  });

  let focusing_power = boxes.focusing_power();

  println!("focusing power: {}", Color::Red(focusing_power));
}

pub fn run() {
  print_test();
  let test_str = read_input("day_15/test");
  part_one(&test_str);
  part_two(&test_str);

  println!();

  print_solution();
  let input = read_input("day_15/input");
  part_one(&input);
  part_two(&input);
}
