pub struct Pattern {
  pub mirror_map: Vec<Vec<char>>,
}
impl Pattern {
  pub fn new(chart: &String) -> Pattern {
    let mirror_map = chart.lines().map(|line| {
      line.chars().collect()
    }).collect();

    Pattern {
      mirror_map,
    }
  }
}
