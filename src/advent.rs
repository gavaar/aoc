use crate::shared::YEAR;

mod y2023;
mod y2024;

pub fn run_day(buffer: &str) {
  match YEAR {
    2023 => {
      y2023::run_day(buffer);
    }
    2024 => {
      y2024::run_day(buffer);
    }
    _ => {
      panic!("That year was not made");
    }
  }
}
