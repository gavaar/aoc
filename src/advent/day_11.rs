use crate::shared::{print_test, print_solution, read_input, Color};

mod universe;
use universe::Universe;

fn part_one(chart: &String) {
  let mut universe = Universe::new(chart);
  universe.expand(1);
  println!(
    "the distance between galaxies is {}",
    Color::Red(universe.distance_between_galaxies()),
  );
}

fn part_two(chart: &String, age: usize) {
  let mut universe = Universe::new(chart);
  universe.expand(age);
  println!(
    "the distance between galaxies is {}",
    Color::Red(universe.distance_between_galaxies()),
  );
}

pub fn run() {
  print_test();
  let chart = read_input("day_11/test");
  part_one(&chart);
  part_two(&chart, 9);
  part_two(&chart, 99);

  println!();

  print_solution();
  let chart = read_input("day_11/input");
  part_one(&chart);
  // add the previous value of 1 plus 999_999 to make the new 1_000_000
  part_two(&chart, 999_999);
}
