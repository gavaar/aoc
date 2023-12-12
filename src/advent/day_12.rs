use crate::shared::{print_test, print_solution, read_input, Color};

fn pound_values(line: &str) -> Vec<usize> {
  let mut values = Vec::new();
  let mut count = 0;
  let mut chars = line.chars();

  let check_for_num = |count: &mut usize, values: &mut Vec<usize>| {
    if *count > 0 {
      values.push(*count);
      *count = 0;
    }
  };

  loop {
    let char = chars.next();

    match char {
      Some('.') => {
        check_for_num(&mut count, &mut values);
      }
      Some('#') => {
        count += 1;
      }
      _ => {
        check_for_num(&mut count, &mut values);
        break;
      }
    }
  }

  values
}

fn possible_solutions(line: &str) -> Vec<String> {
  let mut solutions = vec![String::new()];

  for char in line.chars() {
    match char {
      '.' | '#' => {
        for solution in solutions.iter_mut() {
          solution.push(char);
        }
      }
      '?' => {
        let solutions_len = solutions.len();
        let mut new_solutions: Vec<String> = Vec::with_capacity(solutions_len);

        for solution in solutions.iter_mut() {
          let mut new_sol = solution.clone();
          new_sol.push('.');
          solution.push('#');
          new_solutions.push(new_sol);
        }

        solutions = vec![solutions, new_solutions].concat();
      }
      _ => unreachable!(),
    }
  }

  solutions
}

fn part_one(input: &String) {
  let lines = input.lines();

  let mut accepted_solutions: Vec<usize> = Vec::with_capacity(input.lines().count());

  for line in lines {
    let (decode, expected_str) = line.split_once(' ').expect("broke when splittin");
    let expected = expected_str.split(',').map(|v| v.parse::<usize>().expect("expected value broke when usize")).collect::<Vec<usize>>();

    let possible: Vec<String> = possible_solutions(decode).into_iter().filter_map(|s| {
      let values = pound_values(s.as_str());

      if values == expected {
        return Some(s);
      }

      None
    }).collect();

    accepted_solutions.push(possible.len());
  }

  println!("the possible solution sum is {}", Color::Red(accepted_solutions.iter().sum::<usize>()));
}

pub fn run() {
  print_test();
  let input = read_input("day_12/test");
  println!("-- p1");
  part_one(&input);
  
  println!();
  
  print_solution();
  println!("-- p1");
  let input = read_input("day_12/input");
  part_one(&input);

}
