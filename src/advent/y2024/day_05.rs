use page_updater::PageUpdater;

use crate::shared::{print_solution, print_test, read_input, Color};

mod page_updater;

fn parse_input(uri: &str) -> PageUpdater {
  let input = read_input(uri);
  PageUpdater::new(&input)
}

fn both_parts(updater: &PageUpdater) {
  let (correct_mid_pages, corrected_mid_pages) = updater.correct_middle_pages();

  println!("{}", Color::Green("Part One"));
  println!("Sum of mid pages is {}", Color::Blue(correct_mid_pages.iter().sum::<u32>()));
  println!("{}", Color::Green("Part Two"));
  println!("Sum of mid pages is {}", Color::Blue(corrected_mid_pages.iter().sum::<u32>()));
}

pub fn run() {
  print_test();
  let test_updater = parse_input("day_05/test");
  both_parts(&test_updater);
  println!();

  print_solution();
  let updater = parse_input("day_05/input");
  both_parts(&updater);
}
