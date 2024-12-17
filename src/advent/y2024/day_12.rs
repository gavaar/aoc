mod farm;

use crate::shared::{print_solution, print_test, read_input, Color};
use farm::Farm;

fn part_one(farm: &mut Farm) {
  farm.plant_grid();
  let sum: u32 = farm.farmed_plots.iter().map(|plot| plot.price()).sum();
  println!("The total price is {}", Color::Blue(sum));
}

pub fn run() {
  print_test();
  let test_1 = read_input("day_12/test_1");
  let mut test_farm = Farm::new(&test_1);
  part_one(&mut test_farm);
  let test_2 = read_input("day_12/test_2");
  let mut test_farm = Farm::new(&test_2);
  part_one(&mut test_farm);
  let test_3 = read_input("day_12/test_3");
  let mut test_farm = Farm::new(&test_3);
  part_one(&mut test_farm);
  println!();

  print_solution();
  let input = read_input("day_12/input");
  let mut farm = Farm::new(&input);
  part_one(&mut farm);
}
