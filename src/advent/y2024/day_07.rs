mod operation;

use operation::Operation;
use crate::shared::{print_solution, print_test, read_input, report_progress};

fn build_operations(input: &String) -> Vec<Operation> {
  input.lines().map(|line| Operation::new(line)).collect()
}

fn part_one(operations: &mut Vec<Operation>) {
  let progress_total = operations.len();
  operations.iter_mut().enumerate().for_each(|(progress_current, op)| {
    report_progress(progress_current, progress_total);
    op.find_solutions();
  });
  let valid_test: u128 = operations.iter().filter(|op| op.is_valid()).map(|op| op.expected_result).sum();
  println!("\nThe sum of valid tests is {valid_test}");
}

fn part_two(operations: &mut Vec<Operation>) {
  let progress_total = operations.len() * 3;
  operations.iter_mut().enumerate().for_each(|(progress_current, op)| {
    op.reset();
    report_progress(progress_current, progress_total);
  });

  operations.iter_mut().enumerate().for_each(|(progress_current, op)| {
    op.insert_char('|');
    report_progress(progress_current, progress_total);
  });

  operations.iter_mut().enumerate().for_each(|(progress_current, op)| {
    op.find_solutions();
    report_progress(progress_current, progress_total);
  });
  let valid_test: u128 = operations.iter().filter(|op| op.is_valid()).map(|op| op.expected_result).sum();
  println!("\nThe sum of valid tests is {valid_test}");
}

pub fn run() {
  print_test();
  let test_input = read_input("day_07/test");
  let mut test_operations = build_operations(&test_input);
  part_one(&mut test_operations);
  part_two(&mut test_operations);
  println!();

  print_solution();
  let input = read_input("day_07/input");
  let mut operations = build_operations(&input);
  part_one(&mut operations);
  part_two(&mut operations);
}
