use std::collections::HashSet;

struct Coord(usize, usize);
impl Coord {
  pub fn cmp(&self, coord: &Coord) -> usize {
    coord.1.abs_diff(self.1) + coord.0.abs_diff(self.0)
  }

  pub fn move_x(&mut self, val: usize) {
    self.1 += val;
  }
  pub fn move_y(&mut self, val: usize) {
    self.0 += val;
  }
}

pub struct Universe {
  galaxies: Vec<Coord>,
  empty: (Vec<usize>, Vec<usize>),
}
impl Universe {
  pub fn new(chart: &String) -> Universe {
    let mut galaxies = Vec::new();
    let mut empty_rows: HashSet<usize> = HashSet::from_iter(0..chart.lines().count());
    let mut empty_cols: HashSet<usize> = HashSet::from_iter(0..chart.lines().next().unwrap().len());

    for (r, line) in chart.lines().enumerate() {
      for (c, char) in line.chars().enumerate() {
        if char == '#' {
          galaxies.push(Coord(r, c));
          empty_rows.remove(&r);
          empty_cols.remove(&c);
        }
      }
    }

    let mut empty = (
      Vec::from_iter(empty_rows.into_iter()),
      Vec::from_iter(empty_cols.into_iter()),
    );

    empty.0.sort();
    empty.1.sort();

    Universe {
      galaxies,
      empty,
    }
  }

  pub fn expand(&mut self, by_age: usize) {
    let empty_rows = self.empty.0.iter().rev().to_owned();
    let empty_cols = self.empty.1.iter().rev().to_owned();

    for row in empty_rows {
      self.galaxies.iter_mut().for_each(|g| {
        if &g.0 > row {
          g.move_y(by_age);
        }
      });
    }

    for col in empty_cols {
      self.galaxies.iter_mut().for_each(|g| {
        if &g.1 > col {
          g.move_x(by_age);
        }
      });
    }
  }

  pub fn distance_between_galaxies(&self) -> usize {
    let mut sum = 0usize;

    for g_ind_1 in 0..self.galaxies.len() {
      for g_ind_2 in g_ind_1..self.galaxies.len() {
        sum += self.galaxies[g_ind_1].cmp(&self.galaxies[g_ind_2]);
      }
    }

    sum
  }
}
