mod inventory;
use inventory::Inventory;

use crate::shared::{Color, print_solution, print_test, read_input};

fn part_one(uri: &str) {
  let input = read_input(format!("day_05/{uri}").as_str());
  let mut inventory = Inventory::default();
  
  let (fresh_ranges, available) = input.split_once("\n\n").expect("input has no double \n?");

  // add available
  for id_str in available.lines() {
    let id = id_str.parse::<usize>().expect("should be a number");
    inventory.mark_as_available(&id);
  }

  let all_available_ingr = inventory.get_all_ingredients();

  // add fresh
  for range in fresh_ranges.lines() {
    let (start_str, end_str) = range.split_once("-").expect("- should exist");
    let start = start_str.parse::<usize>().expect("should not break");
    let end = end_str.parse::<usize>().expect("should not break");

    for id in &all_available_ingr {
      if id >= &start && id <= &end {
        inventory.mark_as_fresh(id);
      }
    }
  }

  let fresh_ingredients = inventory.fresh_available_ingredients();
  println!("Fresh ingredients count: {}", Color::Green(fresh_ingredients.iter().count()));
}

pub fn run() {
  print_test();
  part_one("test");
  
  println!();
  
  print_solution();
  part_one("input");
}
