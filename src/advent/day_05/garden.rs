use super::source_dest_map::SourceDestMap;

fn get_seeds(line: &str) -> Vec<&str> {
  line.split_once(": ")
    .expect("get_seeds split_once error")
    .1
    .split(' ')
    .collect()
}

pub struct Garden<'a> {
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
