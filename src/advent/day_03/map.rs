use std::collections::HashMap;


fn input_map(input: String) -> (Vec<Vec<char>>, HashMap<(usize, usize), char>, Vec<(String, Vec<(usize, usize)>)>) {
  let cloned = input.clone();
  let mut cloned_lines = cloned.lines();

  let columns = cloned_lines.nth(0).unwrap().len();
  let rows = cloned_lines.count() + 1;

  let mut input_map = vec![vec![' '; columns]; rows];
  let mut symbols = HashMap::new();
  let mut numbers = Vec::new();

  for (rx, line) in cloned.lines().enumerate() {
    let mut new_number: (String, Vec<(usize, usize)>) = (String::new(), Vec::new());

    for (cx, char) in line.chars().enumerate() {
      input_map[rx][cx] = char;

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

  (input_map, symbols, numbers)
}

pub struct Map {
  input: Vec<Vec<char>>,
  symbols: HashMap<(usize, usize), char>,
  numbers: Vec<(String, Vec<(usize, usize)>)>,
  row_len: usize,
  col_len: usize,
}
impl Map {
  pub fn new(input: String) -> Map {
    let (input, symbols, numbers) = input_map(input);
    let row_len = input.len();
    let col_len = input[0].len();

    Map {
      input,
      symbols,
      numbers,
      row_len,
      col_len,
    }
  }

  pub fn valid_numbers(&self) -> (Vec<u64>, HashMap<(usize, usize), Vec<u64>>) {
    let mut valid: Vec<u64> = Vec::new();
    let mut gears: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    for (number_str, num_pos) in &self.numbers {
      let mut pushed = false;

      for (rx, cx) in num_pos {
        for row in 0..3 {
          if rx == &0 && row == 0 { continue; }

          for col in 0..3 {
            if cx == &0 && col == 0 { continue; }

            let check_pos = (rx + row - 1, cx + col - 1);

            if let Some(char) = self.symbols.get(&check_pos) {
              if !pushed {
                let number = number_str.parse::<u64>().expect("something broke when parsing num");

                if char == &'*' {
                  if !gears.contains_key(&check_pos) {
                    gears.insert(check_pos.clone(), Vec::new());
                  }

                  gears.get_mut(&check_pos).unwrap().push(number.clone());
                }

                pushed = true;
                valid.push(number);
              }
            }
          }
        }
      }
    }

    (valid, gears)
  }
}
