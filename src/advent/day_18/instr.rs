use super::dir::Dir;

#[derive(Debug)]
pub struct Instr<'a> {
  pub dir: Dir,
  pub steps: u32,
  hex_color: &'a str,
}
impl<'a> Instr<'a> {
  pub fn new(line: &'a str) -> Instr<'a> {
    let mut lines = line.split(' ').into_iter();

    let dir = match lines.next().expect("should not fail ???") {
      "U" => Dir::Up,
      "R" => Dir::Right,
      "D" => Dir::Down,
      "L" => Dir::Left,
      _ => unreachable!(),
    };
    let steps: u32 = lines.next().expect("should not fail").parse().expect("should not fail");
    let hex_color_with_parens = lines.next().expect("should not fail");
    let hex_color = &hex_color_with_parens[1..hex_color_with_parens.len() - 1];

    Instr {
      dir,
      steps,
      hex_color,
    }
  }

  pub fn apply_hex(&mut self) {
    let letter = self.hex_color.chars().last().unwrap();
    self.dir = match letter {
      '0' => Dir::Right,
      '1' => Dir::Down,
      '2' => Dir::Left,
      '3' => Dir::Up,
      _ => unreachable!(), //shold be
    };
    let hex_code = &self.hex_color[1..self.hex_color.len() - 1];
    self.steps = u32::from_str_radix(hex_code, 16).expect("should not fail");
  }
}
