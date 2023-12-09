use crate::shared::{print_test, print_solution, read_input};

fn pattern(numbers: &Vec<i32>) -> i32 {
  let mut patterns = Vec::new();

  for i in 1..numbers.len() {
    let prev = numbers[i - 1];
    let curr = numbers[i];

    patterns.push(prev - curr);
  }

  if patterns.iter().all(|i| i == &0) {
    return patterns[0];
  }

  return patterns[0] + pattern(&patterns);
}

fn sum_of_next_nums(input: &String) -> i32 {
  input.lines().map(|line| {
    let numbers: Vec<i32> = line.split(' ').rev().map(|v| v.parse::<i32>().unwrap()).collect();
    let num_pattern = pattern(&numbers);
    let next_num = numbers[0] + num_pattern;
    println!("numbers 0 is {} and pattern {}", numbers[0], num_pattern);
    println!("next num is {}", next_num);
    next_num
  }).sum::<i32>()
}

pub fn run() {
  print_test();
  let input = read_input("day_09/test");
  let sum = sum_of_next_nums(&input);
  println!("the sum is {}", sum);

  println!();

  print_solution();
  let input = read_input("day_09/input");
  let sum = sum_of_next_nums(&input);
  println!("the sum is {}", sum);
}
