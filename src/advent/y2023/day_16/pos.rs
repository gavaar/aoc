use std::ops::AddAssign;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Pos(pub isize, pub isize);
impl AddAssign for Pos {
  fn add_assign(&mut self, other: Self) {
    *self = Self(self.0 + other.0, self.1 + other.1);
  }
}
impl PartialEq for Pos {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1
  }
}
impl Eq for Pos {}
impl Pos {
  pub fn directed(direction: &str) -> Pos {
    match direction {
      "top" => Pos(-1, 0),
      "bot" => Pos(1, 0),
      "right" => Pos(0, 1),
      "left" => Pos(0, -1),
      _ => unreachable!(),
    }
  }
}
