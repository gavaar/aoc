use std::collections::HashMap;

pub const TOP_LEFT: &str = "top-left";
pub const TOP: &str = "top";
pub const TOP_RIGHT: &str = "top-right";
pub const LEFT: &str = "left";
pub const RIGHT: &str = "right";
pub const BOT_LEFT: &str = "bot-left";
pub const BOT: &str = "bot";
pub const BOT_RIGHT: &str = "bot-right";

pub struct Grid {
  pub positions: HashMap<(usize, usize), char>,
}
impl Grid {
  pub fn new(input: &String) -> Grid {
    let mut positions = HashMap::new();
    input.lines().enumerate()
      .for_each(|(y, line)| {
        line.chars().enumerate()
          .for_each(|(x, char)| {
            positions.insert((x, y), char);
          });
      });

    Grid { positions }
  }

  pub fn single_adjacent(&self, pos_with_dir: (usize, usize, &str)) -> Option<(usize, usize, char)> {
    let x = match pos_with_dir.2 {
      /* left */ LEFT | TOP_LEFT | BOT_LEFT => {
        if pos_with_dir.0 == 0 {
          return None
        }

        pos_with_dir.0 - 1
      }
      /* right */ RIGHT | BOT_RIGHT | TOP_RIGHT => {
        pos_with_dir.0 + 1
      }
      _ => pos_with_dir.0,
    };
    let y = match pos_with_dir.2 {
      /* top */ TOP_LEFT | TOP | TOP_RIGHT => {
        if pos_with_dir.1 == 0 {
          return None
        }
        pos_with_dir.1 - 1
      }
      /* bot */ BOT_LEFT | BOT | BOT_RIGHT => {
        pos_with_dir.1 + 1
      }
      _ => pos_with_dir.1
    };

    if let Some(adj_char) = self.positions.get(&(x, y)) {
      return Some((x, y, *adj_char));
    }
    None
  }

  pub fn adjacents(&self, pos: &(usize, usize)) -> Vec<(usize, usize, &str, char)> {
    let mut adjacents = Vec::new();

    for dir in [TOP_LEFT, TOP, TOP_RIGHT, LEFT, RIGHT, BOT_LEFT, BOT, BOT_RIGHT].iter() {
      if let Some(adj_value) = self.single_adjacent((pos.0, pos.1, *dir)) {
        adjacents.push(( adj_value.0, adj_value.1, *dir, adj_value.2));
      }
    }

    adjacents
  }
}
