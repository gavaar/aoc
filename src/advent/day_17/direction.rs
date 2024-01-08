
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
  Top,
  Right,
  Bot,
  Left,
}
impl Direction {
  pub fn next(&self, where_to: &str) -> Self {
    match where_to {
      "forward" => self.forward(),
      "left" => self.left(),
      "right" => self.right(),
      "back" => self.back(),
      _ => unreachable!(),
    }
  }

  fn back(&self) -> Direction {
    match self {
      Direction::Top => Direction::Bot,
      Direction::Right => Direction::Left,
      Direction::Bot => Direction::Top,
      Direction::Left => Direction::Right,
    }
  }

  fn forward(&self) -> Direction {
    match self {
      Direction::Top => Direction::Top,
      Direction::Right => Direction::Right,
      Direction::Bot => Direction::Bot,
      Direction::Left => Direction::Left,
    }
  }

  fn left(&self) -> Direction {
    match self {
      Direction::Top => Direction::Left,
      Direction::Right => Direction::Top,
      Direction::Bot => Direction::Right,
      Direction::Left => Direction::Bot,
    }
  }

  fn right(&self) -> Direction {
    match self {
      Direction::Top => Direction::Right,
      Direction::Right => Direction::Bot,
      Direction::Bot => Direction::Left,
      Direction::Left => Direction::Top,
    }
  }
}
