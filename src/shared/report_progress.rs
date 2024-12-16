use std::{io::{stdout, Write}, time::Instant};
use once_cell::sync::OnceCell;

use crate::shared::Color;
static START_TIME: OnceCell<Instant> = OnceCell::new();

pub fn report_progress(current: usize, total: usize) {
  let start_time = START_TIME.get_or_init(Instant::now);
  let one_percent: f32 = (total as f32 - 1f32) / 100f32;
  let current_percent = (current as f32 / one_percent).ceil() as usize;
  let left = 100 - current_percent;
  let progress = format!("{}% [{}{}{}]", current_percent, Color::Green("=".repeat(current_percent)), Color::Green(">"), " ".repeat(left));
  let current_plus_one = current + 1;

  print!("\r{progress} @ {current_plus_one} / {total}");
  print!(" ({}ms)", Color::Blue(start_time.elapsed().as_millis()));

  stdout().flush().unwrap();

  if current == total - 1 {
    println!();
  }
}
