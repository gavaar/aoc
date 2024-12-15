pub mod drawable;

pub trait Map2D {
  fn grid_map(&self) -> &Vec<Vec<char>>;

  fn build_from_char_text(str_ref: &String) -> Vec<Vec<char>> {
    let mut grid_map = Vec::new();
    str_ref.lines().for_each(|line| {
      let mut row = Vec::new();

      line.chars().for_each(|char| {
        row.push(char);
      });

      grid_map.push(row);
    });
    grid_map
  }

  fn get(&self, pos: &(usize, usize)) -> Option<&char> {
    self.grid_map().get(pos.1).and_then(|row| row.get(pos.0))
  }

  fn get_at_dir(&self, curr: &(usize, usize), dir: &str) -> Option<(usize, usize, char)> {
    match dir {
      "left" => {
        if curr.0 == 0 { return None };
        let Some(pos_value) = self.get(&(curr.0 - 1, curr.1)) else {
          return None;
        };
        Some((curr.0 - 1, curr.1, *pos_value))
      }
      "right" => {
        let Some(pos_value) = self.get(&(curr.0 + 1, curr.1)) else {
          return None;
        };
        Some((curr.0 + 1, curr.1, *pos_value))
      }
      "top" => {
        if curr.1 == 0 { return None };
        let Some(pos_value) = self.get(&(curr.0, curr.1 - 1)) else {
          return None;
        };
        Some((curr.0, curr.1 - 1, *pos_value))
      }
      "bot" => {
        let Some(pos_value) = self.get(&(curr.0, curr.1 + 1)) else {
          return None;
        };
        Some((curr.0, curr.1 + 1, *pos_value))
      }
      _ => panic!("dir has to be either: top, left, right, or bot"),
    }
  }
}
