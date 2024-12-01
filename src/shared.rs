use std::fs;

mod color;
mod report_progress;
pub use color::Color;
pub use report_progress::report_progress;
pub const YEAR: u16 = 2024;

pub fn read_input(uri: &str) -> String {
  let url = format!("src/advent/y{YEAR}/{}", uri);
  fs::read_to_string(url).expect("error reading file")
}

pub fn print_test() {
  println!("##### {} #####", Color::Yellow("TEST"));
}

pub fn print_solution() {
  println!("### {} ###", Color::Green("SOLUTION"));
}

pub trait ToU8Map {
  fn to_u8_map(&self) -> Vec<Vec<u8>>;
}
impl ToU8Map for String {
  fn to_u8_map(&self) -> Vec<Vec<u8>> {
    self.lines().map(|line| {
      line.chars().map(|c| {
        c.to_digit(10).unwrap() as u8
      }).collect()
    }).collect()
  }
}