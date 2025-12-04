use std::fmt::Display;

pub enum Map2DDirection { N, NE, E, SE, S, SW, W, NW }
impl Map2DDirection {
  pub fn get_vert_horiz_dirs() -> [Map2DDirection; 4] {
    [
      Map2DDirection::N,
      Map2DDirection::E,
      Map2DDirection::S,
      Map2DDirection::W,
    ]
  }

  pub fn get_all_dirs() -> [Map2DDirection; 8] {
    [
      Map2DDirection::N,
      Map2DDirection::NE,
      Map2DDirection::E,
      Map2DDirection::SE,
      Map2DDirection::S,
      Map2DDirection::SW,
      Map2DDirection::W,
      Map2DDirection::NW,
    ]
  }
}
impl Display for Map2DDirection {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Map2DDirection::N => write!(f, "N"),
        Map2DDirection::NE => write!(f, "NE"),
        Map2DDirection::E => write!(f, "E"),
        Map2DDirection::SE => write!(f, "SE"),
        Map2DDirection::S => write!(f, "S"),
        Map2DDirection::SW => write!(f, "SW"),
        Map2DDirection::W => write!(f, "W"),
        Map2DDirection::NW => write!(f, "NW"),
      }
  }
}
