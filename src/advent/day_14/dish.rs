pub struct Dish {
  dish_map: Vec<Vec<char>>,
}
impl Dish {
  fn swap_pos(&mut self, from: (usize, usize), to: (usize, usize)) {
    let new_from_value = self.dish_map[to.0][to.1];
    let new_to_value = self.dish_map[from.0][to.1];

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

  pub fn print(&self) {
    for row in &self.dish_map {
      for col in row {
        print!("{col}");
      }
      print!("\n");
    }
  }
}
