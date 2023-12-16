use super::{pos::Pos, contraption::Contraption};

#[derive(Clone)]
pub struct Beam {
  pub position: Pos,
  // only goes from +1 to -1. Vertical
  pub velocity: Pos,
  pub finished: bool,
}
impl Beam {
  pub fn duplicate(from: &Beam) -> Beam {
    let new_velocity = match from.direction() {
      "top" => {
        Pos::directed("bot")
      }
      "right" => {
        Pos::directed("left")
      },
      _ => unreachable!(),
    };

    Beam {
      position: from.position,
      velocity: new_velocity,
      finished: false,
    }
  }

  pub fn react_to_env(&mut self, contraption: &Contraption) -> bool /* duplicates */ {
    let space = contraption.get_char(self.position);

    match space {
      Some('.') => {
        false
      }
      Some('/') => {
        match self.direction() {
          "right" => {
            self.velocity = Pos::directed("top");
          }
          "left" => {
            self.velocity = Pos::directed("bot");
          }
          "top" => {
            self.velocity = Pos::directed("right");
          }
          "bot" => {
            self.velocity = Pos::directed("left");
          },
          _ => unreachable!(),
        }

        false
      }
      Some('\\') => {
        match self.direction() {
          "right" => {
            self.velocity = Pos::directed("bot");
          }
          "left" => {
            self.velocity = Pos::directed("top");
          }
          "top" => {
            self.velocity = Pos::directed("left");
          }
          "bot" => {
            self.velocity = Pos::directed("right");
          },
          _ => unreachable!(),
        }

        false
      }
      Some('|') => {
        if self.is_vertical() {
          return false;
        }
        // always goes top, so it dupes to bot
        self.velocity = Pos::directed("top");
        true
      }
      Some('-') => {
        if self.is_horizontal() {
          return false;
        }

        // always right, so it dupes to left
        self.velocity = Pos::directed("right");
        true
      }
      None => {
        self.finished = true;
        false
      }
      _ => unreachable!(),
    }
  }

  pub fn try_move(&mut self, contr: &mut Contraption) {
    if contr.history.contains(&(self.velocity, self.position)) {
      self.finished = true;
    }
    if self.finished {
      return;
    }
    contr.history.push((self.velocity, self.position));
    self.moved();
  }

  fn moved(&mut self) {
    self.position += self.velocity;
  }

  fn direction(&self) -> &str {
    // will never go diagonal
    if self.velocity.0 == 1 {
      return "bot";
    }
    if self.velocity.0 == -1 {
      return "top";
    }
    if self.velocity.1 == 1 {
      return "right";
    }
    if self.velocity.1 == -1 {
      return "left";
    }
    unreachable!();
  }

  fn is_horizontal(&self) -> bool {
    self.velocity.0 == 0
  }

  fn is_vertical(&self) -> bool {
    self.velocity.1 == 0
  }
}
impl Default for Beam {
  fn default() -> Self {
      Beam {
        position: Pos(0, 0),
        velocity: Pos(0, 1),
        finished: false,
      }
  }
}
