use grid::{Grid, BOT_LEFT, BOT_RIGHT, TOP_LEFT, TOP_RIGHT};

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

fn part_two(grid: &Grid) {
  let mut xmas_count = 0;

  for position in &grid.positions {
    if position.1 == &'A' {
      let Some(top_left) = grid.single_adjacent((position.0.0, position.0.1, TOP_LEFT)) else { continue; };
      let Some(top_right) = grid.single_adjacent((position.0.0, position.0.1, TOP_RIGHT)) else { continue; };
      let Some(bot_left) = grid.single_adjacent((position.0.0, position.0.1, BOT_LEFT)) else { continue; };
      let Some(bot_right) = grid.single_adjacent((position.0.0, position.0.1, BOT_RIGHT)) else { continue; };

      let top_left_to_right_is_mas = (top_left.2 == 'M' && bot_right.2 == 'S') || (top_left.2 == 'S' && bot_right.2 == 'M');
      let bot_left_to_right_is_mas = (bot_left.2 == 'M' && top_right.2 == 'S') || (bot_left.2 == 'S' && top_right.2 == 'M');

      if top_left_to_right_is_mas && bot_left_to_right_is_mas {
        xmas_count += 1;
      }
    }
  }

  println!("XMAS count is {}", Color::Blue(xmas_count));
}

pub fn run() {
  print_test();
  let test_grid = build_grid("day_04/test");
  part_one(&test_grid);
  part_two(&test_grid);
  println!();

  print_solution();
  let grid = build_grid("day_04/input");
  part_one(&grid);
  part_two(&grid);
}
