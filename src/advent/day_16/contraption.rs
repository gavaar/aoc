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

  pub fn size(&self) -> (usize, usize) {
    (self.contraption_map.len(), self.contraption_map[0].len())
  }

  pub fn energized(&self) -> usize {
    let mut painted_points: Vec<Pos> = Vec::new();
    self.history.iter().for_each(|point| {
      if !painted_points.contains(&point.1) {
        painted_points.push(point.1);
      }
    });

    painted_points.iter().count()
  }
}
