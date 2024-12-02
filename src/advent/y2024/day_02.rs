mod report;

use crate::shared::{print_solution, print_test, read_input, Color};
use report::Report;

fn get_reports(uri: &str) -> Vec<Report> {
  let files = read_input(uri);
  files.lines().map(|line| Report::new(line)).collect()
}

fn part_one(reports: &Vec<Report>) {
  let safe_reports = reports.iter().filter(|report| report.is_safe()).count();
  println!("There are {} safe reports", Color::Blue(safe_reports));
}

pub fn run() {
  print_test();
  let reports = get_reports("day_02/test");
  part_one(&reports);
  println!();
  
  print_solution();
  let reports = get_reports("day_02/input");
  part_one(&reports);
}
