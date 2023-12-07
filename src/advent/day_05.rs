use crate::shared::{print_test, print_solution, read_input, Color};

fn get_seeds(line: &str) -> Vec<&str> {
  line.split_once(": ")
    .expect("get_seeds split_once error")
    .1
    .split(' ')
    .collect()
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

  pub fn destination_comes_from(&self, value: u64) -> Option<u64> {
    if value >= self.dest_start && value <= self.dest_start + (self.src_end - self.src_start) {
      let diff = value - self.dest_start;
      return Some(self.src_start + diff);
    };

    None
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
  seeds: Vec<&'a str>,
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

  // find the seed that would end up in the given value
  pub fn reverse_find(&self, final_location: u64) -> Option<u64> {
    let mut current_value = final_location;

    'step: for step in (0..7).rev() {
      let current_step = &self.steps[step];

      for source_dest_map in current_step {
        if let Some(matched) = source_dest_map.destination_comes_from(current_value) {
          current_value = matched;
          continue 'step;
        }
      }
    }

    let mut seeds = self.seeds.clone().into_iter();

    loop {
      let seed_start_opt = seeds.next();
      let seed_size_opt = seeds.next();
      
      if seed_start_opt.is_none() || seed_size_opt.is_none() {
        return None;
      }

      let seed_start = seed_start_opt.unwrap().parse::<u64>().unwrap();
      let seed_size = seed_size_opt.unwrap().parse::<u64>().unwrap();
      let seed_end = seed_start + seed_size;
      
      if current_value >= seed_start && current_value <= seed_end {
        return Some(current_value);
      }
    }
  }

  pub fn find_locations(&self) -> Vec<u64> {
    let mut locations = Vec::new();
    
    self.seeds.clone().into_iter().for_each(|seed| {
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
  println!("lowest location is: {}", Color::Red(locations.iter().min().unwrap()));
}

fn part_two(uri: &str) {
  let almanac = read_input(uri);
  let garden = Garden::new(&almanac);

  let mut smaller_location = 0;
  
  let found_seed = loop {
    if let Some(found_seed) = garden.reverse_find(smaller_location) {
      break found_seed;
    } else {
      smaller_location += 1;
    }
  };

  println!("starting seed for smaller location ({}): {}\n", smaller_location, found_seed);
}

pub fn run() {
  print_test();
  part_one("day_05/test");
  part_two("day_05/test");
  
  print_solution();
  part_one("day_05/input");
  part_two("day_05/input");
}
