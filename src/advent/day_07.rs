use crate::shared::{print_test, print_solution, read_input, Color};

mod play;
use play::Play;

fn parse_input(input: &String, joker: bool) -> Vec<Play> {
  input.lines().map(|line| Play::new(line, joker)).collect()
}

fn sum_of_winnings(plays: &Vec<Play>) -> u32 {
  plays.iter().enumerate().map(|(ind, play)| {
    (ind as u32 + 1) * play.bid
  }).sum()
}

fn part_one(input: &String) {
  let mut plays = parse_input(input, false);
  plays.sort_by(|a, b| a.cmp(b));
  let winnings = sum_of_winnings(&plays);

  println!("winnings: {}", Color::Red(winnings));
}

fn part_two(input: &String) {
  let mut plays = parse_input(input, true);
  plays.sort_by(|a, b| a.cmp(b));
  let winnings = sum_of_winnings(&plays);

  println!("winnings with joker: {}\n", Color::Red(winnings));
}

pub fn run() {
  print_test();
  let input = read_input("day_07/test");
  part_one(&input);
  part_two(&input);
  
  print_solution();
  let input = read_input("day_07/input");
  part_one(&input);
  part_two(&input);
}
