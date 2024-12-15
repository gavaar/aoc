use std::fmt::Display;

pub struct StoneHistory {
  pub current_stones: Vec<String>,
  history: Vec<Vec<String>>,
}
impl StoneHistory {
  pub fn new(input: &String) -> StoneHistory {
    StoneHistory {
      current_stones: input.split(" ").map(|v| v.to_string()).collect(),
      history: Vec::new(),
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
      let mut new_stone_arragenment: Vec<String> = Vec::new();
      self.current_stones.iter().for_each(|stone| {
        let mut new_stones = StoneHistory::transform_stone(stone);
        new_stone_arragenment.append(&mut new_stones);
      });
      self.history.push(new_stone_arragenment.clone());
      self.current_stones = new_stone_arragenment;
    }
  }
}

impl Display for StoneHistory {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.current_stones.join(" "))
  }
}