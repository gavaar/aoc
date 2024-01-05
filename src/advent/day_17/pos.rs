use super::direction::Direction;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Pos(pub usize, pub usize);
impl Pos {
  pub fn next(&self, dir: &Direction) -> Option<Pos> {
    match dir {
      Direction::Top => {
        if self.0 == 0 {
          return None;
        }

        Some(Pos(self.0 - 1, self.1))
      }
      Direction::Left => {
        if self.1 == 0 {
          return None;
        }

        Some(Pos(self.0, self.1 - 1))
      }
      Direction::Bot => Some(Pos(self.0 + 1, self.1)),
      Direction::Right => Some(Pos(self.0, self.1 + 1)),
    }
  }
}
