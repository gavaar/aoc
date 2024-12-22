mod machine;

use machine::Machine;

use crate::shared::{print_solution, print_test, read_input, Color};

fn get_machines(uri: &str) -> Vec<Machine> {
  read_input(uri).split("\n\n").map(|machine_str| {
    Machine::new(machine_str)
  }).collect()
}

fn part_one(machines: &Vec<Machine>) {
  let sum: u64 = machines.iter().map(|mac| mac.win_cost()).sum();
  println!("The cheapest winning tokens cost {}", Color::Blue(sum));
}

pub fn run() {
  print_test();
  let test_machines = get_machines("day_13/test");
  part_one(&test_machines);

  println!();
  print_solution();
  let machines = get_machines("day_13/input");
  part_one(&machines);
}
