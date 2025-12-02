use crate::shared::{Color, print_test, read_input, report_progress};

fn is_number_invalid(number: u128) -> bool {
  let num_str = number.to_string();
  let (first_half, second_half) = num_str.split_at(num_str.len() / 2);

  return first_half == second_half;
}

fn get_wrong_ranges(start: u128, end: u128, report: &mut (usize, usize)) -> Vec<u128> {
  let mut wrong_ranges = Vec::new();

  for i in start..=end {
    report_progress(report.0, report.1);
    report.0 += 1;

    if is_number_invalid(i) {
      wrong_ranges.push(i);
    }
  }

  wrong_ranges
}

fn part_one(file_uri: &str) {
  let ids_ranges_str = read_input(file_uri);
  let id_ranges: Vec<(u128, u128)> = ids_ranges_str.split(",")
    .map(|range| {
      let (x_str, y_str) = range.split_once("-").expect("range could not be split with -");
      (
        x_str.parse().expect("tried to parse a non-number"),
        y_str.parse().expect("tried to parse a non-number"),
      )
    }).collect();

    let total: u128 = id_ranges.iter().map(|(start, end)| 1 + end - start).sum::<u128>();
    let mut report = (0usize, total as usize);
  
  let mut total_wrong_ranges: Vec<u128> = Vec::new();
  
  for (start, end) in id_ranges {
    let mut wrong_ranges = get_wrong_ranges(start, end, &mut report);
    total_wrong_ranges.append(&mut wrong_ranges);
  }

  println!("The sum of wrong ranges is: {}", Color::Green(total_wrong_ranges.iter().sum::<u128>()));
}

pub fn run() {
  print_test();
  part_one("day_02/test");
  println!();

  part_one("day_02/input");
}
