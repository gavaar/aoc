use std::collections::HashMap;

use super::card::Card;

pub struct Stack {
  cards: HashMap<usize, Card>,
  pub card_amount: HashMap<usize, usize>,
}
impl Stack {
  pub fn new(card_table: &String) -> Stack {
    let mut card_amount = HashMap::new();
    let mut cards = HashMap::new();

    card_table.lines().for_each(|row| {
      let card = Card::new(row);
      card_amount.insert(card.id.clone(), 1);
      cards.insert(card.id.clone(), card);
    });

    Stack {
      cards,
      card_amount,
    }
  }

  pub fn run_instructions(&mut self) {
    let mut sorted_cards: Vec<&usize> = self.cards.keys().collect();
    sorted_cards.sort();

    for card_id in sorted_cards {
      let Some(card) = self.cards.get(card_id) else {
        return println!("I cant fail tbh");
      };

      let score = card.winning_score();
      let matches = if score == 0 { 0 } else { score.ilog2() + 1 };
      let self_card_amount = self.card_amount.get(card_id).unwrap().to_owned();
      
      for i in (card_id + 1)..(card_id + 1 + matches as usize) {
        if let Some(prev) = self.card_amount.get(&i) {
          self.card_amount.insert(i, prev + self_card_amount);
        } else {
          
        };
      }
    }
  }
}
