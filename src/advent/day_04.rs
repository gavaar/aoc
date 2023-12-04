use crate::{
  shared::{read_input, print_test, print_solution, Color},
  advent::day_04::{stack::Stack, card::Card}
};

mod card;
mod stack;

fn part_two(card_table: &String) {
  let mut stack = Stack::new(card_table);
  stack.run_instructions();

  let card_sum: usize = stack.card_amount.values().sum();
  println!("card sum is: {}\n", Color::Red(card_sum));
}

fn part_one(card_table: &String) {
  let cards = card_table.lines().map(|row| Card::new(row));

  let score_sum: u32 = cards.map(|card| card.winning_score()).sum();
  println!("The score of these cards is: {}",  Color::Red(score_sum));
}

pub fn run() {
  print_test();
  let test_card_table = read_input("day_04/test");
  part_one(&test_card_table);
  part_two(&test_card_table);
  
  print_solution();
  let input_card_table = read_input("day_04/input");
  part_one(&input_card_table);
  part_two(&input_card_table);
}
