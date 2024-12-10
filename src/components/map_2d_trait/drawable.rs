use std::fmt::Display;

use crate::shared::Color;

pub trait Drawable {
  fn matrix_to_paint(&self) -> &Vec<Vec<impl Display>>;

  fn color_override(&self, value: impl Display, _pos: &(usize, usize)) -> Color<impl Display> {
    Color::Default(value)
  }

  fn draw(&self) {
    println!();
    for (row, row_value) in self.matrix_to_paint().iter().enumerate() {
      for (col, value) in row_value.iter().enumerate() {
        print!("{}", self.color_override(value,&(col, row)));
      }
      println!();
    }
    println!();
  }
}
