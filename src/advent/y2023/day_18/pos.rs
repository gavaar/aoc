use super::dir::Dir;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Pos(pub usize, pub usize);
impl Pos {
  pub fn next(&self, dir: &Dir) -> Option<Pos> {
    match dir {
      Dir::Up => {
        if self.0 == 0 {
          return None;
        }

        Some(Pos(self.0 - 1, self.1))
      }
      Dir::Left => {
        if self.1 == 0 {
          return None;
        }

        Some(Pos(self.0, self.1 - 1))
      }
      Dir::Down => Some(Pos(self.0 + 1, self.1)),
      Dir::Right => Some(Pos(self.0, self.1 + 1)),
    }
  }
}
