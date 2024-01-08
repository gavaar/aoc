use super::{pos::Pos, direction::Direction};

pub struct Node {
  pub pos: Pos,
  pub direction: Direction,
  pub overheat: usize,
  pub heat_loss: u32,
  pub path: Vec<Pos>,
}
impl Default for Node {
  fn default() -> Self {
    Node {
      pos: Pos(0, 0),
      direction: Direction::Right,
      overheat: 0,
      heat_loss: 0,
      path: vec![Pos(0, 0)],
    }
  }
}
