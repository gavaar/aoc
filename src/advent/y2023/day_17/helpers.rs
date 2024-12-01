use crate::shared::Color;

use super::pos::Pos;

pub fn draw(heat_map: &Vec<Vec<u8>>, path: &Vec<Pos>) {
  let mut str_vec: Vec<Vec<String>> = heat_map.iter().map(|row| row.iter().map(|heat_loss| heat_loss.to_string()).collect()).collect();

  for pos in path {
    let original_value = str_vec[pos.0][pos.1].to_owned();
    str_vec[pos.0][pos.1] = Color::Green(original_value).to_string();
  }

  let mut drawing = String::new();
  for row in str_vec {
    let mut line = String::new();
    for col in row {
      line.push_str(col.as_str());
    }
    line.push('\n');
    drawing.push_str(line.as_str());
  }

  println!();
  println!("{drawing}");
}
