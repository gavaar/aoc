use std::collections::HashMap;

pub struct Operation {
  pub expected_result: u128,
  operands: Vec<char>,
  components: Vec<u128>,
  solutions: HashMap<String, u128>,
}
impl Operation {
  pub fn new(line: &str) -> Operation {
    let (expected, numbers) = line.split_once(": ").expect("should never fail");

    Operation {
      expected_result: expected.parse().expect("should be num"),
      components: numbers.split(" ").map(|s| s.parse().unwrap()).collect(),
      operands: Vec::from(['+', '*']),
      solutions: HashMap::new(),
    }
  }

  pub fn find_solutions(&mut self) {
    let mut components_iter = self.components.iter();
    let first = components_iter.next().unwrap();
    self.solutions.insert(String::new(), *first);

    while let Some(next) = components_iter.next() {
      let mut prev_step_solutions = self.solutions.keys();
      let mut new_solutions = HashMap::new();

      while let Some(solution) = prev_step_solutions.next() {
        for oper in &self.operands {
          let value = self.solutions.get(solution).unwrap();
          let next_oper = format!("{solution}{oper}");
          let new_result = match oper {
            '+' => value + next,
            '*' => value * next,
            '|' => format!("{value}{next}").parse().expect("concat should not fail"),
            _ => panic!("should not reach"),
          };
          new_solutions.insert(next_oper, new_result); 
        }
      }

      self.solutions = new_solutions;
    }
  }

  pub fn reset(&mut self) {
    self.solutions = HashMap::new();
  }

  pub fn insert_char(&mut self, char: char) {
    self.operands.push(char);
  }

  pub fn is_valid(&self) -> bool {
    let result = self.solutions.iter().find(|&(_sol, value)| {
      return value == &self.expected_result;
    });

    if result.is_some() {
      return true;
    }

    return false
  }
}
