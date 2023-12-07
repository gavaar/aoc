use crate::shared::{read_input, print_test, print_solution, Color};

fn quadratic_equation(a: f64, b: f64, c: f64) -> (f64, f64) {
  let inner = b * b - 4f64 * a * c;
  let lower = 2f64 * a;

  (
    (-1f64 * b + inner.sqrt()) / lower,
    (-1f64 * b - inner.sqrt()) / lower,
  )
}

#[derive(Debug)]
pub struct Race {
  pub record: f64,
  pub total_time: f64,
}
impl Race {
  /*
    time_pressed = x;
    total_time = b
    distance = c;

               [        time left        ]
    distance = (total_time - time_pressed) time_pressed; // time_pressed === speed;
    c = (b - x) x;

    0 = -x^2 + bx - c.
  */
  pub fn breakpoints_to_break_record(&self) -> (f64, f64) {
    quadratic_equation(-1f64, self.total_time, -1f64 * self.record)
  }
}

fn parse_input(input: String) -> Vec<Race> {
  let mut split_input = input.lines().into_iter();
  let times: Vec<f64> = split_input.next().unwrap()
    .split_once(':').unwrap()
    .1
    .split(' ')
    .filter(|v| !v.is_empty())
    .map(|v| v.parse::<f64>().unwrap())
    .collect();
  let records: Vec<f64> = split_input.next().unwrap()
    .split_once(':').unwrap()
    .1
    .split(' ')
    .filter(|v| !v.is_empty())
    .map(|v| v.parse::<f64>().unwrap())
    .collect();

  let mut race_vec = Vec::new();

  for i in 0..times.len() {
    race_vec.push(Race {
       record: records[i],
       total_time: times[i],
    });
  }

  race_vec
}

fn next_down(f: f64) -> u64 {
  (f - 1f64).ceil() as u64
}
fn next_up(f: f64) -> u64 {
  (f + 1f64).floor() as u64
}
fn number_of_ways_to_break_record(races: Vec<Race>) -> Vec<u64> {
  races.iter().map(|race| {
    let (start, end) = race.breakpoints_to_break_record();
    (next_down(end) - next_up(start)) as u64 + 1
  }).collect()
}

fn part_one(input: &str) {
  let input = read_input(input);
  let races = parse_input(input);
  let Some(num_of_ways) = number_of_ways_to_break_record(races).into_iter().reduce(|acc, e| acc * e) else {
    return println!("something broke when multiplying the amount of solutions for each race");
  };
  println!("num of ways multiplier is {}", Color::Red(num_of_ways));
}

fn clean_input_whitespaces(input: String) -> String {
  let mut clean = String::new();

  for line in input.lines() {
    let collected: String = line.split_whitespace().collect();
    clean.push_str(collected.as_str());
    clean.push('\n');
  }

  clean
}

fn part_two(input: &str) {
  let input = read_input(input);
  let clean = clean_input_whitespaces(input);
  let races = parse_input(clean);

  let Some(num_of_ways) = number_of_ways_to_break_record(races).into_iter().reduce(|acc, e| acc * e) else {
    return println!("something broke when multiplying the amount of solutions for each race");
  };

  println!("clean multiplier is {}\n", Color::Red(num_of_ways));
}

pub fn run() {
  print_test();
  part_one("day_06/test");
  part_two("day_06/test");
  
  print_solution();
  part_one("day_06/input");
  part_two("day_06/input");
}
