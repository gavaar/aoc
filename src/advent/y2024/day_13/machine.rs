use std::fmt::Display;
use regex::Regex;

fn get_button_values(from: &str) -> (i128, i128) {
  let button_regex = Regex::new(r"X[+=](\d+),\sY[+=](\d+)").expect("regex error");
  let result = button_regex.captures(from).unwrap();

  let x = result.get(1).unwrap().as_str().parse().unwrap();
  let y = result.get(2).unwrap().as_str().parse().unwrap();

  (x, y)
}

pub struct Machine {
  a: (i128, i128),
  b: (i128, i128),
  prize: (i128, i128),
}
impl Machine {
  pub fn new(machine_str: &str) -> Machine {
    let mut lines = machine_str.lines();

    let a = get_button_values(lines.next().unwrap());
    let b = get_button_values(lines.next().unwrap());
    let prize = get_button_values(lines.next().unwrap());
    Machine { a, b, prize }
  }

  pub fn correct(&mut self) {
    let corrected_a = self.prize.0 + 10000000000000;
    let corrected_b = self.prize.1 + 10000000000000;

    self.prize = (corrected_a, corrected_b);
  }

  // 0 means no win
  pub fn win_cost(&self, limit: bool) -> u128 {
    // system of equations formula
    if (self.a.0 * self.prize.1 - self.a.1 * self.prize.0) % (self.a.0 * self.b.1 - self.a.1 * self.b.0) != 0 {
      return 0;
    }
    let b_presses = (self.a.0 * self.prize.1 - self.a.1 * self.prize.0) / (self.a.0 * self.b.1 - self.a.1 * self.b.0);

    if (self.prize.0 - self.b.0 * b_presses) % self.a.0 != 0 {
      return 0;
    }
    
    let a_presses = (self.prize.0 - self.b.0 * b_presses) / self.a.0;
    
    if limit && (a_presses > 100 || b_presses > 100) {
      return 0;
    }

    (a_presses * 3 + b_presses) as u128
  }
}

impl Display for Machine {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, r"Machine {{ A: X+{} Y+{}; B: X+{} Y+{}; Prize: X={} Y={} }}", self.a.0, self.a.1, self.b.0, self.b.1, self.prize.0, self.prize.1)
  }
}
