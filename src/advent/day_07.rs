use crate::shared::{print_test, print_solution, read_input, Color};

mod play;
use play::Play;

fn parse_input(input: &String) -> Vec<Play> {
  input.lines().map(|line| Play::new(line)).collect()
}

fn part_one(input: String) {
  let mut plays = parse_input(&input);
  plays.sort_by(|a, b| a.cmp(b));
  let total_winnings = plays.iter().enumerate().map(|(ind, play)| {
    (ind as u32 + 1) * play.bid
  }).sum::<u32>();

  println!("winnings: {}\n", Color::Red(total_winnings));
}

pub fn run() {
  print_test();
  let input = read_input("day_07/test");
  part_one(input);
  
  print_solution();
  let input = read_input("day_07/input");
  part_one(input);
}