use crate::shared::{print_test, print_solution, read_input, Color, report_progress};

mod mirror;
mod pattern;
mod util;
pub use pattern::Pattern;

fn mirrors(all_mirrors: &String) -> Vec<Pattern> {
  all_mirrors.split("\n\n").map(|m| {
    Pattern::new(&m.to_string())
  }).collect()
}

fn part_one(all_mirrors: &String) {
  let mirrors = mirrors(all_mirrors);
  let solution = mirrors.iter().map(|mirror| {
    let solutions = util::search_for_equals(&mirror.mirror_map);
    solutions[0].value()
  }).sum::<u32>();

  println!("mirror sum: {}", Color::Red(solution));
}

fn part_two(all_mirrors: &String) {
  let patterns = mirrors(all_mirrors);
  let total = patterns.len();
  let mut smudged_solutions = Vec::new();

  for (curr, mirror) in patterns.iter().enumerate() {
    report_progress(curr, total);

    let solutions = util::search_for_smudge(&mirror);
    smudged_solutions.push(solutions.value());
  }

  println!("mirror sum: {}", Color::Red(smudged_solutions.iter().sum::<u32>()));
}

pub fn run() {
  print_test();
  let mirrors_test = read_input("day_13/test");
  println!("-- p1");
  part_one(&mirrors_test);
  println!("-- p2");
  part_two(&mirrors_test);

  println!();
  
  print_solution();
  let mirrors = read_input("day_13/input");
  println!("-- p1");
  part_one(&mirrors);
  println!("-- p2");
  part_two(&mirrors);
}
