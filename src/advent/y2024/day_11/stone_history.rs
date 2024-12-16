use std::collections::HashMap;

use crate::shared::report_progress;

pub struct StoneHistory {
  pub current_stones: HashMap<String, u128>,
}
impl StoneHistory {
  pub fn new(input: &String) -> StoneHistory {
    let mut current_stones = HashMap::new();

    input.split(" ").for_each(|num| {
      current_stones.insert(num.to_string(), 1);
    });
    StoneHistory {
      current_stones,
    }
  }

  fn transform_stone(stone: &String) -> Vec<String> {
    if stone == &"0".to_string() {
      return vec!["1".to_string()];
    }

    let char_count = stone.chars().count();
    if char_count % 2 == 0 {
      let (stone_left, stone_right) = stone.split_at(char_count / 2);
      let left = stone_left.trim_start_matches("0").to_string();
      let right = stone_right.trim_start_matches("0").to_string();

      return vec![
        if left == "" { "0".to_string() } else { left.to_string() },
        if right == "" { "0".to_string() } else { right.to_string() },
      ];
    }

    let new_stone = stone.parse::<u128>().unwrap() * 2024;
    return vec![new_stone.to_string()]
  }

  pub fn blink(&mut self, amount: usize) {
    for _i in 0..amount {
      report_progress(_i, amount);
      let mut new_stone_arragenment: HashMap<String, u128> = HashMap::new();

      self.current_stones.iter().for_each(|(stone, amount)| {
        let new_stones = StoneHistory::transform_stone(stone);

        for new_stone in new_stones {
          let old_value = *new_stone_arragenment.get(&new_stone).unwrap_or(&0);
          new_stone_arragenment.insert(new_stone, old_value + amount);
        }
      });

      self.current_stones = new_stone_arragenment;
    }
  }
}
