mod list_holder;

use crate::shared::{print_solution, print_test, read_input, Color};
use list_holder::ListHolder;

fn list_holder_from_uri(uri: &str) -> ListHolder {
  let input = read_input(uri);
  ListHolder::new(&input)
}

fn part_one(list_holder: &ListHolder) {
  let diff_vectors = list_holder.get_diffs_vector();
  println!("Sum of differences is {}", Color::Blue(diff_vectors.iter().sum::<u32>()));
}

fn part_two(list_holder: &ListHolder) {
  let similarity_score = list_holder.similarity_score();
  println!("The similarity score is {}", Color::Blue(similarity_score));
}

pub fn run() {
  print_test();
  let test_holder = list_holder_from_uri("day_01/test");
  part_one(&test_holder);
  part_two(&test_holder);
  println!("");
  
  print_solution();
  let list_holder = list_holder_from_uri("day_01/input");
  part_one(&list_holder);
  part_two(&list_holder);
  println!("");
}
