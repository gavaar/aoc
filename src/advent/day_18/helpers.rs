use std::collections::HashMap;

use super::{instr::Instr, pos::Pos, dir::Dir};

pub fn build_perimeter(instr: Vec<Instr>, starting_pos: Pos) -> HashMap<usize, Vec<usize>>{
  let mut map = HashMap::new();
  let mut needle = starting_pos;

  for instruct in instr.iter() {    
    for _idx in 0..instruct.steps {
      needle = needle.next(&instruct.dir).expect("should not break");
      if map.get(&needle.0).is_none() {
        map.insert(needle.0, vec![]);
      }
  
      map.get_mut(&needle.0).unwrap().push(needle.1);
    }
  }

  map
}

// instructions / starting_pos
pub fn instructions_and_starting_pos(input: &String, use_hex: bool) -> (Vec<Instr>, Pos) {
  let mut up = 0i128;
  let mut down = 0i128;
  let mut left = 0i128;
  let mut right = 0i128;
  let mut vertical = 0i128;
  let mut horizontal = 0i128;

  let instructions = input.lines().map(|l| {
    let mut instr = Instr::new(l);
    if use_hex {
      instr.apply_hex();
    }

    match instr.dir {
      Dir::Up => {
        vertical -= instr.steps as i128;

        if vertical < up {
          up = vertical;
        }
      }
      Dir::Down => {
        vertical += instr.steps as i128;

        if vertical > down {
          down = vertical;
        }
      }
      Dir::Right => {
        horizontal += instr.steps as i128;

        if horizontal > right {
          right = horizontal;
        }
      }
      Dir::Left => {
        horizontal -= instr.steps as i128;

        if horizontal < left {
          left = horizontal;
        }
      }
    }

    instr
  }).collect();

  let starting_pos = Pos((-1 * up) as usize, (-1 * left) as usize);

  return (instructions, starting_pos);
}

pub fn angle(map: &HashMap<usize, Vec<usize>>, row: usize, col: usize) -> &str {
  if row == 0 {
    return "bot";
  }

  if row == map.len() - 1 {
    return "top";
  }

  let top = map.get(&(row - 1)).unwrap().contains(&col);
  let bot = map.get(&(row + 1)).unwrap().contains(&col);
  
  if top && bot {
    return "both";
  }

  if top {
    return "top";
  }

  if bot {
    return "bot";
  }

  unreachable!(); // should be
}

pub fn draw(points: &HashMap<usize, Vec<usize>>) {
  let max_cols = points.iter().map(|(_, val)| { val.iter() }).flatten().max().expect("bruh");

  let mut drawing = String::new();

  for row in 0..points.len() {
    let mut line = String::new();
    for col in 0..*max_cols + 1 {
      // line.push(if points.get(&Pos(row, col)).is_some() { '#' } else { '.' });
      line.push(if points.get(&row).unwrap().contains(&col) { '#' } else { '.' });
    }
    line.push('\n');
    drawing.push_str(line.as_str());
  }

  println!();
  println!("{drawing}");
}
