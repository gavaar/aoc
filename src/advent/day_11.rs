use std::collections::HashSet;

use crate::shared::{print_test, print_solution, read_input, Color};

#[derive(Debug, Clone)]
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

#[derive(Debug)]
struct Universe {
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

  pub fn expand(&mut self, vecchio: bool) {
    let empty_rows = self.empty.0.iter().rev().to_owned();
    let empty_cols = self.empty.1.iter().rev().to_owned();

    for row in empty_rows {
      self.galaxies.iter_mut().for_each(|g| {
        if &g.0 > row {
          // add the previous value of 1 plus 999_999 to make the new 1_000_000
          g.move_y(if vecchio { 999_999 } else { 1 });
        }
      });
    }

    for col in empty_cols {
      self.galaxies.iter_mut().for_each(|g| {
        if &g.1 > col {
          // add the previous value of 1 plus 999_999 to make the new 1_000_000
          g.move_x(if vecchio { 999_999 } else { 1 });
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

fn part_one(chart: &String) {
  let mut universe = Universe::new(chart);
  universe.expand(false);
  println!(
    "the distance between galaxies is {}",
    Color::Red(universe.distance_between_galaxies()),
  );
}

fn part_two(chart: &String) {
  let mut universe = Universe::new(chart);
  universe.expand(true);
  println!(
    "the distance between galaxies is {}",
    Color::Red(universe.distance_between_galaxies()),
  );
}

pub fn run() {
  print_test();
  let chart = read_input("day_11/test");
  part_one(&chart);
  part_two(&chart);

  println!();

  print_solution();
  let chart = read_input("day_11/input");
  part_one(&chart);
  part_two(&chart);
}
