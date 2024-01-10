use super::dir::Dir;

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
}
