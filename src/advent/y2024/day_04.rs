use grid::Grid;

use crate::shared::{print_solution, print_test, read_input, Color};

mod grid;

fn build_grid(uri: &str) -> Grid {
  let input = read_input(uri);
  Grid::new(&input)
}

fn part_one(grid: &Grid) {
  let mut xmas_count = 0;

  for position in &grid.positions {
    if position.1 == &'X' {
      let adjacents = grid.adjacents(&position.0);

      for adj in adjacents {
        if adj.3 == 'M' {
          if let Some(expecting_a) = grid.single_adjacent((adj.0, adj.1, adj.2)) {
            if expecting_a.2 == 'A' {
              if let Some(expecting_s) = grid.single_adjacent((expecting_a.0, expecting_a.1, adj.2)) {
                if expecting_s.2 == 'S' {
                  xmas_count += 1;
                }
              }
            }
          }
        }
      }
    }
  }

  println!("XMAS count is {}", Color::Blue(xmas_count));
}

pub fn run() {
  print_test();
  let test_grid = build_grid("day_04/test");
  part_one(&test_grid);
  println!();

  print_solution();
  let grid = build_grid("day_04/input");
  part_one(&grid);
}
