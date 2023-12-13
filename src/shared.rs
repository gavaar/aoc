use std::{fs, io::{stdout, Write}};

mod color;
pub use color::Color;

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

pub fn report_progress(current: usize, total: usize) {
  let one_percent: f32 = (total as f32 - 1f32) / 100f32;
  let current_percent = (current as f32 / one_percent).ceil() as usize;
  let left = 100 - current_percent;
  let progress = format!("[{}{}{}]", Color::Green("=".repeat(current_percent)), Color::Green(">"), " ".repeat(left));

  print!("\r{progress}");
  stdout().flush().unwrap();
}
