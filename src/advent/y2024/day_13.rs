mod machine;

use machine::Machine;

use crate::shared::{print_solution, print_test, read_input, Color};

fn get_machines(uri: &str) -> Vec<Machine> {
  read_input(uri).split("\n\n").map(|machine_str| {
    Machine::new(machine_str)
  }).collect()
}

fn part_one(machines: &Vec<Machine>) {
  let sum: u128 = machines.iter().map(|mac| mac.win_cost(true)).sum();
  println!("The cheapest winning tokens cost {}", Color::Blue(sum));
}

fn part_two(machines: &mut Vec<Machine>) {
  let sum: u128 = machines.iter_mut().map(|mac| {
    mac.correct();
    mac.win_cost(false)
  }).sum();
  println!("The cheapest winning tokens cost {}", Color::Blue(sum));
}

pub fn run() {
  print_test();
  let mut test_machines = get_machines("day_13/test");
  part_one(&test_machines);
  part_two(&mut test_machines);
  
  println!();
  print_solution();
  let mut machines = get_machines("day_13/input");
  part_one(&machines);
  part_two(&mut machines);
}
