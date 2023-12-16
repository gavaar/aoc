use super::pos::Pos;

pub struct Contraption {
  contraption_map: Vec<Vec<char>>,
  // (velocity, position)
  pub history: Vec<(Pos, Pos)>,
}
impl Contraption {
  pub fn new(input: &String) -> Contraption {
    let contraption_map = input.lines().map(|line| line.chars().collect()).collect();

    Contraption {
      contraption_map,
      history: Vec::new(),
    }
  }

  pub fn get_char(&self, pos: Pos) -> Option<&char> {
    self.contraption_map.get(pos.0 as usize).and_then(|row| row.get(pos.1 as usize))
  }

  pub fn paint(&self, positions: &Vec<Pos>) -> String {
    let mut paint: Vec<Vec<char>> = Vec::new();

    for row in 0..self.contraption_map.len() {
      paint.push(Vec::new());

      for _col in 0..self.contraption_map[0].len() {
        paint[row].push('.');
      }

      paint[row].push('\n');
    }

    positions.iter().for_each(|pos| {
      paint[pos.0 as usize][pos.1 as usize] = '#';
    });

    paint.iter().map(|lines| lines.iter().collect::<String>()).collect()
  }
}
