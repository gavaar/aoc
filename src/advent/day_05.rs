use std::str::Split;

use crate::shared::{print_test, print_solution, read_input, Color};

fn get_seeds(line: &str) -> Split<char> {
  line.split_once(": ")
    .expect("get_seeds split_once error")
    .1
    .split(' ')
}

#[derive(Debug)]
struct SourceDestMap {
  src_start: u64,
  src_end: u64,
  dest_start: u64,
}
impl SourceDestMap {
  pub fn new(line: &str) -> SourceDestMap {
    let mut split_line = line.split(' ');
    let dest_start = split_line.next()
      .expect("SourceDestMap dest_start error")
      .parse::<u64>()
      .expect("SourceDestMap dest_start parse error");
    let src_start = split_line.next()
      .expect("SourceDestMap src_start error")
      .parse::<u64>()
      .expect("SourceDestMap src_start parse error");
    let size = split_line.next()
      .expect("SourceDestMap size error")
      .parse::<u64>()
      .expect("SourceDestMap size parse error");
    let src_end = src_start + size - 1; // make the end inclusive, that way range is [s..e]

    SourceDestMap { src_start, src_end, dest_start }
  }

  pub fn next_destination(&self, current: u64) -> Option<u64> {
    if current >= self.src_start && current <= self.src_end {
      let diff = current - self.src_start;
      return Some(self.dest_start + diff);
    }

    None
  }
}

struct Garden<'a> {
  seeds: Split<'a, char>,
  steps: [Vec<SourceDestMap>; 7],
}
impl<'a> Garden<'a> {
  pub fn new(almanac: &'a String) -> Garden<'a> {
    let mut lines = almanac.lines();
    let seeds = get_seeds(lines.next().expect("parse_input get_seeds error"));
    let mut steps: [Vec<SourceDestMap>; 7] = [
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
    ];
    lines.nth(1); // skip blank line plus first step's name
    let mut current_step = 0usize;

    for line in lines {
      if line.is_empty() { continue; }
      if line.contains("map:") {
        current_step += 1;
        continue;
      }

      steps[current_step].push(SourceDestMap::new(line));
    }

    Garden {
      seeds,
      steps,
    }
  }

  pub fn find_locations(&self) -> Vec<u64> {
    let mut locations = Vec::new();
    
    self.seeds.clone().for_each(|seed| {
      let mut current_number = seed.parse::<u64>()
        .expect("find_locations current_number parse error");
  
      'steps: for step in 0..7 {
        let current_step = &self.steps[step];

        for source_dest_map in current_step {
          if let Some(matched) = source_dest_map.next_destination(current_number) {
            current_number = matched;
            continue 'steps;
          }
        }
      }

      // should be solved
      locations.push(current_number);
    });

    locations
  }
}

fn part_one(uri: &str) {
  let almanac = read_input(uri);
  let garden = Garden::new(&almanac);
  let locations = garden.find_locations();
  println!("lowest location is: {}\n", Color::Red(locations.iter().min().unwrap()));
}

pub fn run() {
  print_test();
  part_one("day_05/test");

  print_solution();
  part_one("day_05/input");
}
