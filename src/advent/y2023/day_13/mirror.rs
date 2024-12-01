
#[derive(Debug, Hash)]
pub enum Mirror {
  Vert(usize),
  Hori(usize),
}
impl Mirror {
  pub fn value(&self) -> u32 {
    match self {
      Self::Vert(pos) => (pos * 100) as u32,
      Self::Hori(pos) => *pos as u32,
    }
  }

  pub fn name(&self) -> &str {
    match self {
      Self::Vert(_) => "vertical",
      Self::Hori(_) => "horizontal",
    }
  }
}
impl PartialEq for Mirror {
  fn eq(&self, other: &Self) -> bool {
    self.name() == other.name() && self.value() == other.value()
  }
}
impl Eq for Mirror {}
