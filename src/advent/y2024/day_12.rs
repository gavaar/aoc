mod farm;

use crate::shared::{print_solution, print_test, read_input, Color};
use farm::Farm;

fn part_one(farm: &mut Farm) {
  farm.plant_grid();
  let sum: u32 = farm.farmed_plots.iter().map(|plot| plot.price()).sum();
  println!("The total price is {}", Color::Blue(sum));
}

fn part_two(farm: &mut Farm) {
  let sum: u32 = farm.farmed_plots.iter_mut().map(|p| {
    p.calc_sides();
    p.side_price()
  }).sum();
  println!("The reduced price is {}", Color::Blue(sum));
  println!("------");
}

pub fn run() {
  print_test();
  let test_1 = read_input("day_12/test_1");
  let mut test_farm = Farm::new(&test_1);
  part_one(&mut test_farm);
  part_two(&mut test_farm);
  let test_2 = read_input("day_12/test_2");
  let mut test_farm = Farm::new(&test_2);
  part_one(&mut test_farm);
  part_two(&mut test_farm);
  let test_3 = read_input("day_12/test_3");
  let mut test_farm = Farm::new(&test_3);
  part_one(&mut test_farm);
  part_two(&mut test_farm);
  println!();
  
  print_solution();
  let input = read_input("day_12/input");
  let mut farm = Farm::new(&input);
  part_one(&mut farm);
  part_two(&mut farm);
}
