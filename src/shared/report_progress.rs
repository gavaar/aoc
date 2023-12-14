use std::io::{stdout, Write};

use crate::shared::Color;

pub fn report_progress(current: usize, total: usize) {
  let one_percent: f32 = (total as f32 - 1f32) / 100f32;
  let current_percent = (current as f32 / one_percent).ceil() as usize;
  let left = 100 - current_percent;
  let progress = format!("[{}{}{}]", Color::Green("=".repeat(current_percent)), Color::Green(">"), " ".repeat(left));

  print!("\r{progress} @ {current}");
  stdout().flush().unwrap();

  if left == 0 {
    println!();
  }
}
