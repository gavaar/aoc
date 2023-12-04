use std::fmt::Display;

pub enum Color <Content>
where
  Content: Display
{
  Yellow(Content),
  Green(Content),
  Red(Content),
  Blue(Content),
}
impl<T: Display> Display for Color<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      &Color::Yellow(content) => {
        write!(f, "{}{}{}", "\x1b[92m", content, "\x1b[39m")
      }
      &Color::Green(content) => {
        write!(f, "{}{}{}", "\x1b[32m", content, "\x1b[39m")
      }
      &Color::Red(content) => {
        write!(f, "{}{}{}", "\x1b[31m", content, "\x1b[39m")
      }
      &Color::Blue(content) => {
        write!(f, "{}{}{}", "\x1b[36m", content, "\x1b[39m")
      }
    }
  }
}
