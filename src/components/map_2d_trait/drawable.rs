use std::fmt::Display;
use crate::shared::Color;
use super::Map2D;

pub trait Drawable<T: From<char> + Display + Copy>: Map2D<T> {
  fn color_override(&self, value: impl Display, _pos: &(usize, usize)) -> Color<impl Display> {
    Color::Default(value)
  }

  fn draw(&self) {
    println!();
    for (row, row_value) in self.grid_map().iter().enumerate() {
      for (col, value) in row_value.iter().enumerate() {
        print!("{}", self.color_override(value,&(col, row)));
      }
      println!();
    }
    println!();
  }
}
