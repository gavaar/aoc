mod report;

use crate::shared::{print_solution, print_test, read_input, Color};
use report::Report;

fn get_reports(uri: &str) -> Vec<Report> {
  let files = read_input(uri);
  files.lines().map(|line| Report::new(line)).collect()
}

fn part_one(reports: &mut Vec<Report>) {
  let safe_reports = reports.iter_mut().filter_map(|report| {
    if report.is_safe() {
      return Some(true);
    }
    None
  }).count();
  println!("There are {} safe reports", Color::Blue(safe_reports));
}

fn part_two(reports: &mut Vec<Report>) {
  let safe_reports = reports.iter_mut().filter_map(|report| {
    report.activate_dampener();
    if report.is_safe() {
      return Some(true);
    }
    None
  }).count();
  println!("With dampener on: there are {} safe reports", Color::Blue(safe_reports));
}

pub fn run() {
  print_test();
  let mut reports = get_reports("day_02/test");
  part_one(&mut reports);
  part_two(&mut reports);
  println!();
  
  print_solution();
  let mut reports = get_reports("day_02/input");
  part_one(&mut reports);
  part_two(&mut reports);
}
