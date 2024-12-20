use std::collections::{HashMap, HashSet};
use crate::components::Map2D;

pub struct Plot {
  land_letter: char,
  lands: HashSet<(usize, usize)>,
  perimeter: HashSet<String>,
  side_list: Vec<String>,
}
impl Plot {
  pub fn new(land: &char) -> Plot {
    Plot {
      land_letter: *land,
      lands: HashSet::new(),
      perimeter: HashSet::new(),
      side_list: Vec::new(),
    }
  }

  pub fn size(&self) -> usize {
    self.lands.len()
  }

  pub fn price(&self) -> u32 {
    self.perimeter.len() as u32 * self.lands.len() as u32
  }

  pub fn side_price(&self) -> u32 {
    let sides = self.side_list.len() as u32;
    self.lands.len() as u32 * sides
  }

  pub fn calc_sides(&mut self) {
    let mut sides: HashMap<String, Vec<u32>> = HashMap::new();
    self.perimeter.iter().for_each(|perimeter| {
      let (pos, dir) = perimeter.split_once("-").unwrap();
      let (x, y) = pos.split_once(",").unwrap();
      let (pivot_axis, value_axis) = match dir {
        "left" | "right" => (x, y),
        "top" | "bot" => (y, x),
        _ => panic!("should not"),
      };
      let axis_key = format!("{dir}_{pivot_axis}");
      if !sides.contains_key(axis_key.as_str()) {
        sides.insert(axis_key.to_owned(), Vec::new());
      }

      sides.get_mut(&axis_key).unwrap().push(value_axis.parse().unwrap());
    });

    for (side_name, side_lands) in &mut sides {
      side_lands.sort();
      let mut side_idx = 0;
      let mut side_idx_name = format!("{side_name}_0");
      let mut filler = false;
      let mut current_expected = *side_lands.first().unwrap();
      let last = *side_lands.last().unwrap();

      while current_expected <= last {
        if !self.side_list.contains(&side_idx_name) {
          self.side_list.push(side_idx_name.to_owned());
        }

        if !side_lands.contains(&current_expected) && !filler {
          filler = true;
        }
        
        if filler && side_lands.contains(&current_expected) {
          filler = false;
          side_idx += 1;
          side_idx_name = format!("{side_name}_{side_idx}");
          continue;
        }

        current_expected += 1;
      }
    }
  }
}

pub struct Farm {
  grid: Vec<Vec<char>>,
  farmed_pos: HashSet<(usize, usize)>,
  pub farmed_plots: Vec<Plot>,
}
impl Farm {
  pub fn new(input: &String) -> Farm {
    let grid = Farm::build_from_char_text(input);
    let farmed_pos = HashSet::new();
    let farmed_plots = Vec::new();
    Farm { grid, farmed_pos, farmed_plots }
  }

  pub fn plant_grid(&mut self) {
    for x in 0..self.grid.len() {
      for y in 0..self.grid[0].len() {
        let land = self.get(&(x, y)).expect("should never fail");
        let mut possible_plot = Plot::new(land);
        self.plant(&(x, y), &mut possible_plot);

        if possible_plot.size() > 0 {
          self.farmed_plots.push(possible_plot);
        }
      }
    }
  }

  fn plant(&mut self, pos: &(usize, usize), plot: &mut Plot) {
    if self.farmed_pos.contains(pos) {
      return;
    }

    plot.lands.insert((pos.0, pos.1));
    self.farmed_pos.insert((pos.0, pos.1));

    for dir in ["left", "top", "right", "bot"] {
      let next_dir = self.get_at_dir(pos, dir);
      if next_dir.is_none() {
        plot.perimeter.insert(format!("{},{}-{}", pos.0, pos.1, dir));
        continue;
      }
      let (nx, ny, nc) = next_dir.unwrap();
      if nc != plot.land_letter {
        plot.perimeter.insert(format!("{},{}-{}", pos.0, pos.1, dir));
        continue;
      }
      
      self.plant(&(nx, ny), plot);
    }
  }
}

impl Map2D for Farm {
  fn grid_map(&self) -> &Vec<Vec<char>> {
    &self.grid
  }
}
