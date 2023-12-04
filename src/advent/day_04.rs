use crate::{
  shared::read_input,
  advent::day_04::{stack::Stack, card::Card}
};

mod card;
mod stack;

fn part_two(card_table: &String) {
  let mut stack = Stack::new(card_table);
  stack.run_instructions();

  let card_sum: usize = stack.card_amount.values().sum();
  println!("card sum is: {}\n", card_sum);
}

fn part_one(card_table: &String) {
  let cards = card_table.lines().map(|row| Card::new(row));

  let score_sum: u32 = cards.map(|card| card.winning_score()).sum();
  println!("The score of these cards is: {}", score_sum);
}

pub fn run() {
  println!("### TEST ###");
  let test_card_table = read_input("day_04/test");
  part_one(&test_card_table);
  part_two(&test_card_table);
  
  println!("### SOLUTION ###");
  let input_card_table = read_input("day_04/input");
  part_one(&input_card_table);
  part_two(&input_card_table);
}
