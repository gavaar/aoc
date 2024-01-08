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

  print!("\r{progress} @ {current} / {total}");

  if current_percent > 5 {
    let total_time = (total as f64 / current as f64) * start_time.elapsed().as_secs() as f64;
    let time_left = (100.0 * (total_time - start_time.elapsed().as_secs() as f64) / 60.0).trunc() / 100.0;
    print!(" (ETA: ~{}m remaining)", Color::Blue(time_left));
  }
  stdout().flush().unwrap();

  if current == total - 1 {
    println!();
  }
}
