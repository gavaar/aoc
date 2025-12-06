use regex::Regex;

use crate::shared::{Color, print_solution, print_test, read_input};

#[derive(Debug)]
struct MathInput {
  pub problems: Vec<Vec<String>>,
}
impl MathInput {
  pub fn new(input: &String) -> Self {
    let rows = input.lines();
    let mut problems: Vec<Vec<String>> = Vec::new();

    for row in rows {
      let re = Regex::new(r"\s+").expect("correct regex");
      let no_weird_spaces_row = re.replace_all(row, " ");
      let parsed_row = no_weird_spaces_row.trim().split(" ").into_iter();

      for (idx, operand) in parsed_row.enumerate() {
        if problems.get(idx).is_none() {
          problems.insert(idx, Vec::new());
        }

        problems[idx].push(operand.to_string());
      }
    }
    MathInput { problems }
  }

  pub fn new_cephalod(input: &String) -> Self {
    let mut lines = input.lines();
    let mut columns: Vec<String> = Vec::new();

    for line in lines.by_ref().take_while(|line| !line.contains("*") && !line.contains("+")) {
      for (idx, char) in line.chars().rev().enumerate() {
        if columns.get(idx).is_none() {
          columns.insert(idx, String::new());
        }
        columns[idx].push(char);
      }
    }

    let mut operands = input.lines().last().unwrap().split("").filter(|v| !v.trim().is_empty()).collect::<Vec<&str>>();
    operands.reverse();

    let mut current_idx = 0;
    let mut problems: Vec<Vec<String>> = Vec::new();

    for col in columns {
      let trimmed = col.trim();

      if trimmed.is_empty() {
        current_idx += 1;
        continue;
      }

      if problems.get(current_idx).is_none() {
        problems.insert(current_idx, Vec::new());
      }
      problems[current_idx].push(trimmed.to_string());
    }

    for (idx, op) in operands.iter().enumerate() {
      problems[idx].push(op.to_string());
    }

    MathInput { problems }
  }

  pub fn do_math(&self) -> Vec<u128> {
    self.problems.iter()
      .map(|problem| {
        let mut prob = problem.clone();
        let operand = prob.pop().expect("should have an operand");
        return prob.iter()
          .map(|val| val.parse::<u128>().expect("should be num"))
          .reduce(|acc, val| if operand == "*" { acc * val } else { acc + val })
          .expect("should not error");
      }).collect()
  }
}

pub fn run() {
  print_test();
  let test_input = read_input("day_06/test");
  let test_math = MathInput::new(&test_input);
  let test_results = test_math.do_math();
  println!("Homework sum: {}", Color::Blue(test_results.iter().sum::<u128>()));
  let test_ceph_math = MathInput::new_cephalod(&test_input);
  let test_ceph_results = test_ceph_math.do_math();
  println!("Homework sum: {}", Color::Blue(test_ceph_results.iter().sum::<u128>()));
  println!();
  
  print_solution();
  let input = read_input("day_06/input");
  let math = MathInput::new(&input);
  let results = math.do_math();
  println!("Homework sum: {}", Color::Blue(results.iter().sum::<u128>()));
  let ceph_math = MathInput::new_cephalod(&input);
  let ceph_results = ceph_math.do_math();
  println!("Homework sum: {}", Color::Blue(ceph_results.iter().sum::<u128>()));
}
