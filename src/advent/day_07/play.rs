use std::cmp::Ordering;


#[derive(Debug, Clone, Copy)]
enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
  Error,
}
impl HandType {
  pub fn get(hand: &[char; 5]) -> HandType {
    let mut card_duplicates: Vec<(char, usize)> = Vec::new();

    for card in hand {
      let card_search = card_duplicates.iter_mut().find(|c| &c.0 == card);
      if let Some(found) = card_search {
        found.1 = found.1 + 1;
      } else {
        card_duplicates.push((*card, 1));
      }
    }

    match card_duplicates.len() {
      5 => {
        return HandType::HighCard;
      }
      4 => {
        return HandType::OnePair;
      }
      3 => {
        for card in card_duplicates {
          match card.1 {
            3 => return HandType::ThreeOfAKind,
            2 => return HandType::TwoPair,
            _ => {}
          }
        }
        return HandType::Error
      }
      2 => {
        for card in card_duplicates {
          match card.1 {
            4 | 1 => return HandType::FourOfAKind,
            3 | 2 => return HandType::FullHouse,
            _ => {}
          }
        }
        return HandType::Error
      }
      1 => return HandType::FiveOfAKind,
      _ => HandType::Error,
    }
  }
}

#[derive(Debug)]
pub struct Play {
  pub hand: [char; 5],
  pub bid: u32,
  hand_type: HandType,
}
impl Play {
  pub fn new(line: &str) -> Play {
    let (cards, bid_str) = line.split_once(' ').expect("split broke");
    let hand = cards.chars().collect::<Vec<char>>().try_into().expect("cant make a char array from hand");
    let bid = bid_str.parse::<u32>().expect("bid is NaN");
    let hand_type = HandType::get(&hand);

    Play {
      hand,
      bid,
      hand_type,
    }
  }

  pub fn card_value(card: char) -> usize {
    let Some(found) = "23456789TJQKA".chars().enumerate()
      .find(|(_, c)| { card == *c }) else {
        return 0;
      };

    found.0
  }

  pub fn cmp(&self, play: &Play) -> Ordering {
    let self_value = self.hand_type as usize;
    let play_value = play.hand_type as usize;

    if self_value > play_value {
      return Ordering::Greater;
    }

    if self_value < play_value {
      return Ordering::Less;
    }

    for i in 0..self.hand.len() {
      if self.hand[i] == play.hand[i] {
        continue;
      }

      return Play::card_value(self.hand[i]).cmp(&Play::card_value(play.hand[i]));
    }

    return Ordering::Equal;
  }
}
