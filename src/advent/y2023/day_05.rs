use crate::shared::{print_test, print_solution, read_input, Color};

mod garden;
mod source_dest_map;

use garden::Garden;

fn part_one(uri: &str) {
  let almanac = read_input(uri);
  let garden = Garden::new(&almanac);
  let locations = garden.find_locations();
  println!("lowest location is: {}", Color::Red(locations.iter().min().unwrap()));
}

fn part_two(uri: &str) {
  let almanac = read_input(uri);
  let garden = Garden::new(&almanac);

  let mut smaller_location = 0;
  
  let found_seed = loop {
    if let Some(found_seed) = garden.reverse_find(smaller_location) {
      break found_seed;
    } else {
      smaller_location += 1;
    }
  };

  println!("starting seed for smaller location ({}): {}\n", smaller_location, found_seed);
}

pub fn run() {
  print_test();
  part_one("day_05/test");
  part_two("day_05/test");
  
  print_solution();
  part_one("day_05/input");
  part_two("day_05/input");
}
