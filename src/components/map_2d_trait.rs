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
}
