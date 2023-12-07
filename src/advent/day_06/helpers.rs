use super::Race;

fn next_down(f: f64) -> u64 {
  (f - 1f64).ceil() as u64
}
fn next_up(f: f64) -> u64 {
  (f + 1f64).floor() as u64
}
pub fn number_of_ways_to_break_record(races: Vec<Race>) -> Vec<u64> {
  races.iter().map(|race| {
    let (start, end) = race.breakpoints_to_break_record();
    (next_down(end) - next_up(start)) as u64 + 1
  }).collect()
}

pub fn clean_input_whitespaces(input: String) -> String {
  let mut clean = String::new();

  for line in input.lines() {
    let collected: String = line.split_whitespace().collect();
    clean.push_str(collected.as_str());
    clean.push('\n');
  }

  clean
}

pub fn parse_input(input: String) -> Vec<Race> {
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
