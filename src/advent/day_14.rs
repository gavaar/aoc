use std::collections::HashSet;

use crate::shared::{print_test, print_solution, read_input, Color};

mod dish;
use dish::Dish;

fn parse_input(input: &String) -> Dish {
  Dish::new(input)
}

fn part_one(dish: &mut Dish) {
  dish.tilt_north();

  println!("val after tilt: {}", Color::Red(dish.value()));
}

fn part_two(dish: &mut Dish) {
  let mut set_of_solutions = HashSet::new();
  let mut cycle_size = HashSet::new();
  let mut curr_print = dish.get_print();

  while let true = set_of_solutions.insert(curr_print.to_owned()) {
    dish.cycle();
    curr_print = dish.get_print();
  }
  // when false we came back to the start of the cycle

  while let true = cycle_size.insert(curr_print.to_owned()) {
    dish.cycle();
    curr_print = dish.get_print();
  }
  // when false we know cycle size

  let cycle_count = cycle_size.iter().count();
  // how many solutions on the first minus cycle size
  let offset = set_of_solutions.iter().count() - cycle_count;

  let remaining_cycles = (1_000_000_000 - offset) % cycle_count;

  for _ in 0..remaining_cycles {
    dish.cycle();
  }

  println!("it comes back to cycle the positions after {}", cycle_count);
  println!("the offset is {offset}");
  println!("it's value is {}", Color::Red(dish.value()));
}

pub fn run() {
  print_test();
  println!("-- p1");
  let mut dish = parse_input(&read_input("day_14/test"));
  part_one(&mut dish);

  println!("-- p2");
  let mut dish = parse_input(&read_input("day_14/test"));
  part_two(&mut dish);

  println!();
  
  print_solution();
  println!("-- p1");
  let mut dish = parse_input(&read_input("day_14/input"));
  part_one(&mut dish);

  println!("-- p2");
  let mut dish = parse_input(&read_input("day_14/input"));
  part_two(&mut dish);

}
