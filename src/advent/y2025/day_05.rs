mod inventory;
use std::{cmp, collections::HashSet};

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

fn part_two(uri: &str) {
  let input = read_input(format!("day_05/{uri}").as_str());
  let (fresh_ranges_list, _available) = input.split_once("\n\n").expect("input has no double \n?");

  let mut fresh_ranges: HashSet<(usize, usize)> = HashSet::new();
  let original_fresh_list: HashSet<(usize, usize)> = HashSet::from_iter(
    fresh_ranges_list.lines().map(|line| {
      let (start_str, end_str) = line.split_once("-").expect("- should exist");
      let start = start_str.parse::<usize>().expect("should not break");
      let end = end_str.parse::<usize>().expect("should not break");
      (start, end)
    })
  );
  
  original_fresh_list.iter().for_each(|fresh| {
    let mut current = (fresh.0, fresh.1);

    loop {
      let fresh_snapshot = fresh_ranges.to_owned();

      if let Some((to_remove, to_update)) = &fresh_snapshot.iter().find_map(|other_range| {
        // contained or contains
        let current_inside_other = current.0 >= other_range.0 && current.1 <= other_range.1;
        let other_inside_current = current.0 <= other_range.0 && current.1 >= other_range.1;
        let current_overlaps_left = current.0 < other_range.0 && current.1 < other_range.1 && current.1 >= other_range.0;
        let current_overlaps_right = current.0 > other_range.0 && current.1 > other_range.1 && current.0 <= other_range.1;

        if current_inside_other || other_inside_current || current_overlaps_left || current_overlaps_right {
          return Some(
            (
              other_range,
              (cmp::min(current.0, other_range.0), cmp::max(current.1, other_range.1))
            )
          );
        }
        
        return None;
      }) {
        fresh_ranges.remove(to_remove);
        current = (to_update.0, to_update.1);
        continue;
      }

      fresh_ranges.insert(current);
      break;
    }
  });

  let ranges = fresh_ranges.iter().map(|range| range.1 - range.0 + 1).sum::<usize>();

  println!("Range totals: {}", Color::Green(ranges));
}

pub fn run() {
  print_test();
  part_one("test");
  part_two("test");
  
  println!();
  
  print_solution();
  part_one("input");
  part_two("input");
}
