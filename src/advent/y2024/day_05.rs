use page_updater::PageUpdater;

use crate::shared::{print_solution, print_test, read_input, Color};

mod page_updater;

fn parse_input(uri: &str) -> PageUpdater {
  let input = read_input(uri);
  PageUpdater::new(&input)
}

fn part_one(updater: &PageUpdater) {
  let correct_mid_pages = updater.correct_middle_pages();
  println!("correct mid pages are {:?}", correct_mid_pages);
  println!("Sum of mid pages is {}", Color::Blue(correct_mid_pages.iter().sum::<u32>()));
}

pub fn run() {
  print_test();
  let test_updater = parse_input("day_05/test");
  part_one(&test_updater);
  println!();

  print_solution();
  let updater = parse_input("day_05/input");
  part_one(&updater);
}
