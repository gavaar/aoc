use std::fmt::Display;
use regex::Regex;

pub struct Robot {
  velocity: (isize, isize),
  pos: (usize, usize),
  pub teleport_x: usize,
  pub teleport_y: usize,
}
impl Robot {
  pub fn new(line: &str, size: &(usize, usize)) -> Robot {
    let parser = Regex::new(r"p=(\d+),(\d+)\sv=(-?\d+),(-?\d+)").unwrap();
    let (_, [pos_x, pos_y, vel_x, vel_y]) = parser.captures(line).map(|c| c.extract()).expect("??");
    let velocity = (vel_x.parse().unwrap(), vel_y.parse().unwrap());
    let pos = (pos_x.parse().unwrap(), pos_y.parse().unwrap());
    let teleport_x = size.0;
    let teleport_y = size.1;

    Robot { velocity, pos, teleport_x, teleport_y }
  }

  pub fn x(&self) -> usize {
    self.pos.0
  }

  pub fn y(&self) -> usize {
    self.pos.1
  }

  pub fn walk(&mut self, seconds: u32) {
    let moved_x = self.x() as i32 + self.velocity.0 as i32 * seconds as i32;
    let moved_y = self.y() as i32 + self.velocity.1 as i32 * seconds as i32;

    let remainder_x = moved_x % self.teleport_x as i32;
    let remainder_y = moved_y % self.teleport_y as i32;

    self.pos = (
      if remainder_x < 0 { (remainder_x + self.teleport_x as i32) as usize } else { remainder_x as usize },
      if remainder_y < 0 { (remainder_y + self.teleport_y as i32) as usize } else { remainder_y as usize },
    );
  }
}

impl Display for Robot {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Robot {{ ({}, {}) -> ({}, {}) }}", self.x(), self.y(), self.velocity.0, self.velocity.1)
  }
}
