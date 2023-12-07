use crate::shared::{read_input, print_test, print_solution, Color};

fn quadratic_equation(a: f32, b: f32, c: f32) -> (f32, f32) {
  let inner = b * b - 4f32 * a * c;
  let lower = 2f32 * a;

  (
    (-1f32 * b + inner.sqrt()) / lower,
    (-1f32 * b - inner.sqrt()) / lower,
  )
}

pub struct Race {
  pub record: f32,
  pub total_time: f32,
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
  pub fn breakpoints_to_break_record(&self) -> (f32, f32) {
    quadratic_equation(-1f32, self.total_time, -1f32 * self.record)
  }
}

fn parse_input(input: String) -> Vec<Race> {
  let mut split_input = input.lines().into_iter();
  let times: Vec<f32> = split_input.next().unwrap()
    .split_once(':').unwrap()
    .1
    .split(' ')
    .filter(|v| !v.is_empty())
    .map(|v| v.parse::<f32>().unwrap())
    .collect();
  let records: Vec<f32> = split_input.next().unwrap()
    .split_once(':').unwrap()
    .1
    .split(' ')
    .filter(|v| !v.is_empty())
    .map(|v| v.parse::<f32>().unwrap())
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

fn next_down(f: f32) -> u32 {
  (f - 1f32).ceil() as u32
}
fn next_up(f: f32) -> u32 {
  (f + 1f32).floor() as u32
}
fn number_of_ways_to_break_record(races: Vec<Race>) -> Vec<u32> {
  races.iter().map(|race| {
    let (start, end) = race.breakpoints_to_break_record();
    (next_down(end) - next_up(start)) as u32 + 1
  }).collect()
}

fn part_one(input: &str) {
  let input = read_input(input);
  let races = parse_input(input);
  let Some(num_of_ways) = number_of_ways_to_break_record(races).into_iter().reduce(|acc, e| acc * e) else {
    return println!("something broke when multiplying the amount of solutions for each race");
  };
  println!("num of ways multiplier is {}\n", Color::Red(num_of_ways));
}

pub fn run() {
  print_test();
  part_one("day_06/test");
  
  print_solution();
  part_one("day_06/input");
}
