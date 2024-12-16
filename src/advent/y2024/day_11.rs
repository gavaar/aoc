 mod stone_history;
 use stone_history::StoneHistory;

use crate::shared::{print_solution, print_test, read_input, Color};

fn part_one(stone_history: &mut StoneHistory) {
  stone_history.blink(25);
  let sum: u128 = stone_history.current_stones.values().sum();
  println!("There are {} stones", Color::Blue(sum));
}
fn part_two(stone_history: &mut StoneHistory) {
  stone_history.blink(75);
  println!("There are {} stones", Color::Blue(stone_history.current_stones.values().sum::<u128>()));
}

pub fn run() {
  print_test();
  let test = read_input("day_11/test");
  let mut test_stones = StoneHistory::new(&test);
  part_one(&mut test_stones);
  let mut test_stones = StoneHistory::new(&test);
  part_two(&mut test_stones);
  println!();

  print_solution();
  let input = read_input("day_11/input");
  let mut stones = StoneHistory::new(&input);
  part_one(&mut stones);
  let mut stones = StoneHistory::new(&input);
  part_two(&mut stones);
}
