use std::collections::HashMap;

fn input_map(input: String) -> (HashMap<(usize, usize), char>, Vec<(String, Vec<(usize, usize)>)>) {
  let cloned = input.clone();

  let mut symbols = HashMap::new();
  let mut numbers = Vec::new();

  for (rx, line) in cloned.lines().enumerate() {
    let mut new_number: (String, Vec<(usize, usize)>) = (String::new(), Vec::new());

    for (cx, char) in line.chars().enumerate() {
      if char.is_numeric() {
        new_number.0.push(char);
        new_number.1.push((rx, cx));
      } else {
        if char != '.' {
          symbols.insert((rx, cx), char);
        }

        if new_number.0.clone().len() > 0 {
          numbers.push(new_number);
          new_number = (String::new(), Vec::new());
        }
      }
    }

    if new_number.0.clone().len() > 0 {
      numbers.push(new_number);
    }
  }

  (symbols, numbers)
}

pub struct Map {
  symbols: HashMap<(usize, usize), char>,
  numbers: Vec<(String, Vec<(usize, usize)>)>,
}
impl Map {
  pub fn new(input: String) -> Map {
    let (symbols, numbers) = input_map(input);

    Map {
      symbols,
      numbers,
    }
  }

  pub fn valid_numbers(&self) -> Vec<u64> {
    let mut valid: Vec<u64> = Vec::new();

    for (number_str, num_pos) in &self.numbers {
      'outer: for (rx, cx) in num_pos {
        for row in 0..3 {
          if rx == &0 && row == 0 { continue; }

          for col in 0..3 {
            if cx == &0 && col == 0 { continue; }

            let check_pos = (rx + row - 1, cx + col - 1);

            if let Some(_) = self.symbols.get(&check_pos) {
                let number = number_str.parse::<u64>().expect("something broke when parsing num");

                valid.push(number);
                break 'outer;
            }
          }
        }
      }
    }

    valid
  }

  pub fn gears(&self) -> HashMap<(usize, usize), Vec<u64>> {
    let mut gears: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    for (number_str, num_pos) in &self.numbers {
      'outer: for (rx, cx) in num_pos {
        for row in 0..3 {
          if rx == &0 && row == 0 { continue; }

          for col in 0..3 {
            if cx == &0 && col == 0 { continue; }

            let check_pos = (rx + row - 1, cx + col - 1);

            if let Some(char) = self.symbols.get(&check_pos) {
              let number = number_str.parse::<u64>().expect("something broke when parsing num");

              if char == &'*' {
                if !gears.contains_key(&check_pos) {
                  gears.insert(check_pos.clone(), Vec::new());
                }

                gears.get_mut(&check_pos).unwrap().push(number.clone());
                break 'outer;
              }
            }
          }
        }
      }
    }

    gears
  }
}
