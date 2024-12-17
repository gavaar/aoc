use std::collections::HashSet;
use crate::components::Map2D;

#[derive(Debug)]
pub struct Plot {
  land_letter: char,
  lands: HashSet<(usize, usize)>,
  perimeter: HashSet<String>,
}
impl Plot {
  pub fn new(land: &char) -> Plot {
    Plot {
      land_letter: *land,
      lands: HashSet::new(),
      perimeter: HashSet::new(),
    }
  }

  pub fn size(&self) -> usize {
    self.lands.len()
  }

  pub fn price(&self) -> u32 {
    self.perimeter.len() as u32 * self.lands.len() as u32
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
        // let imaginary_pos = match dir {
        //   "left" => format!("{},{}", pos.0 as isize - 1, pos.1),
        //   "right" => format!("{},{}", pos.0 + 1, pos.1),
        //   "top" => format!("{},{}", pos.0, pos.1 as isize - 1),
        //   "bot" => format!("{},{}", pos.0, pos.1 + 1),
        //   _ => panic!("this will never be true"),
        // };
        // plot.perimeter.insert(imaginary_pos);
        plot.perimeter.insert(format!("{},{}-{}", pos.0, pos.1, dir));
        continue;
      }
      let (nx, ny, nc) = next_dir.unwrap();
      if nc != plot.land_letter {
        // plot.perimeter.insert(format!("{},{}", nx, ny));
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
