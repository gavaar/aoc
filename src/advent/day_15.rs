use crate::shared::{print_test, print_solution, read_input, Color};

fn parse_input<'a>(input: &'a String) -> Vec<&'a str> {
  input.split(',')
    .collect()
}

fn init_sequence(seq: &str) -> u32 {
  seq.chars().fold(0u32, |acc, curr_char| {
    let ascii =  curr_char as u8 as u32;
    ((acc + ascii) * 17) % 256u32
  })
}

pub fn run() {
  print_test();
  let test_str = read_input("day_15/test");
  let sum: u32 = parse_input(&test_str)
    .iter()
    .map(|seq| init_sequence(seq))
    .sum();

  println!("the crazy ascii sum is {}", Color::Red(sum));
  
  print_solution();
  let input = read_input("day_15/input");
  let sum: u32 = parse_input(&input)
    .iter()
    .map(|seq| init_sequence(seq))
    .sum();
  println!("the crazy ascii sum is {}", Color::Red(sum));
}
