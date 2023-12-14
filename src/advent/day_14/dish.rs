pub struct Dish {
  dish_map: Vec<Vec<char>>,
}
impl Dish {
  fn swap_pos(&mut self, from: (usize, usize), to: (usize, usize)) {
    let new_from_value = self.dish_map[to.0][to.1];
    let new_to_value = self.dish_map[from.0][from.1];

    self.dish_map[to.0][to.1] = new_to_value;
    self.dish_map[from.0][from.1] = new_from_value;
  }

  pub fn new(input: &String) -> Dish {
    let mut dish_map = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
      let mut line_vec = Vec::new();

      for char in line.chars() {
        line_vec.push(char);
      }

      dish_map.push(line_vec);
    }

    Dish {
      dish_map
    }
  }

  pub fn cycle(&mut self) {
    self.tilt_north();
    self.tilt_west();
    self.tilt_south();
    self.tilt_east();
  }

  pub fn tilt_north(&mut self) {
    let mut topmost_empty_row = vec![0; self.dish_map[0].len()];
    let iterable_rows = &self.dish_map.to_vec();

    for (row, row_chars) in iterable_rows.iter().enumerate() {
      for (col, character) in row_chars.iter().enumerate() {
        match character {
          'O' => {
            let topmost_empty = topmost_empty_row[col];
            if topmost_empty < row {
              self.swap_pos((row, col), (topmost_empty, col));
            }

            topmost_empty_row[col] = topmost_empty + 1;
          }
          '.' => {}
          '#' => {
            topmost_empty_row[col] = row + 1;
          },
          _wat => unreachable!(),
        }
      }
    }
  }

  fn tilt_south(&mut self) {
    self.dish_map.reverse();
    self.tilt_north();
    self.dish_map.reverse();
  }

  fn tilt_west(&mut self) {
    let mut westmost_empty_col = vec![0; self.dish_map.len()];
    let iterable_rows = &self.dish_map.to_vec();

    for col in 0..iterable_rows[0].len() {
      for (row, line) in iterable_rows.iter().enumerate() {
        let character = line[col];

        match character {
          'O' => {
            let westmost_empty = westmost_empty_col[row];
            if westmost_empty < col {
              self.swap_pos((row, col), (row, westmost_empty));
            }

            westmost_empty_col[row] = westmost_empty + 1;
          }
          '.' => {}
          '#' => {
            westmost_empty_col[row] = col + 1;
          },
          _wat => unreachable!(),
        }
      }
    }
  }

  fn tilt_east(&mut self) {
    for row in self.dish_map.iter_mut() {
      row.reverse();
    }
    self.tilt_west();
    for row in self.dish_map.iter_mut() {
      row.reverse();
    }
  }

  pub fn value(&self) -> u64 {
    let base_val = self.dish_map.len();
    let mut values = Vec::new();

    for (row, line) in self.dish_map.iter().enumerate() {
      for character in line.iter() {
        if character == &'O' {
          let value = base_val - row;
          values.push(value as u64);
        }
      }
    }

    values.into_iter().sum::<u64>()
  }

  pub fn get_print(&self) -> String {
    let mut print = String::with_capacity(self.dish_map.len() * self.dish_map[0].len());

    for row in &self.dish_map {
      for col in row {
        print.push(*col);
      }
      print.push('\n');
    }

    print
  }
}
