use crate::shared::read_input;

use self::card::Card;

mod card;

fn part_one(card_table: String) {
  let cards = card_table.lines().map(|row| Card::new(row));

  let score_sum: u32 = cards.map(|card| card.winning_score()).sum();
  println!("The score of these cards is: {}\n", score_sum);
}

pub fn run() {
  println!("### TEST ###");
  let test_card_table = read_input("src/advent/day_04/test");
  part_one(test_card_table);
  
  println!("### SOLUTION ###");
  let input_card_table = read_input("src/advent/day_04/input");
  part_one(input_card_table);
  
  // let solution_card_table = read_input("src/advent/day_04/input");
}
