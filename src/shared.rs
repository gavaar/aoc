use std::fs;

mod color;
mod report_progress;
pub use color::Color;
pub use report_progress::report_progress;

pub fn read_input(uri: &str) -> String {
  let url = String::from("src/advent/") + uri;
  fs::read_to_string(url).expect("error reading file")
}

pub fn print_test() {
  println!("##### {} #####", Color::Yellow("TEST"));
}

pub fn print_solution() {
  println!("### {} ###", Color::Green("SOLUTION"));
}
