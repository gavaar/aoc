pub mod drawable;
pub mod map_2d_direction;

use map_2d_direction::Map2DDirection;

pub trait Map2D<T: From<char> + Copy> {
  fn grid_map(&self) -> &Vec<Vec<T>>;
  fn unit_from_char(input: char) -> T { return input.into() }

  fn build_from_char_text(str_ref: &String) -> Vec<Vec<T>> {
    let mut grid_map = Vec::new();
    str_ref.lines().for_each(|line| {
      let mut row = Vec::new();

      line.chars().for_each(|char| {
        let val = Self::unit_from_char(char);
        row.push(val);
      });

      grid_map.push(row);
    });
    grid_map
  }

  fn get(&self, pos: &(usize, usize)) -> Option<&T> {
    self.grid_map().get(pos.1).and_then(|row| row.get(pos.0))
  }

  fn get_at_dir(&self, curr: &(usize, usize), dir: &Map2DDirection) -> Option<(usize, usize, T)> {
    let next_x = match dir {
      Map2DDirection::NW | Map2DDirection::SW | Map2DDirection::W => {
        if curr.0 == 0 { return None }
        curr.0 - 1
      }
      Map2DDirection::NE | Map2DDirection::SE | Map2DDirection::E => {
        curr.0 + 1
      }
      _ => curr.0
    };
    let next_y = match dir {
      Map2DDirection::NE | Map2DDirection::NW | Map2DDirection::N => {
        if curr.1 == 0 { return None }
        curr.1 - 1
      }
      Map2DDirection::SE | Map2DDirection::SW | Map2DDirection::S => {
        curr.1 + 1
      }
      _ => curr.1
    };

    let Some(pos_value) = self.get(&(next_x, next_y)) else {
      return None;
    };

    Some((next_x, next_y, *pos_value))
  }

  fn rows(&self) -> usize {
    self.grid_map().len()
  }
  fn cols(&self) -> usize {
    self.grid_map().get(0).unwrap_or(&vec![]).len()
  }

  fn find<F: Fn(&T) -> bool>(&self, predicate: F) -> Option<(usize, usize)> {
    for y in 0..self.rows() {
      for x in 0..self.cols() {
        let value = self.get(&(x, y)).expect("should exist");
        if predicate(value) {
          return Some((x, y));
        }
      }
    }
    None
  }
}
