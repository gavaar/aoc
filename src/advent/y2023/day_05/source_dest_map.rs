pub struct SourceDestMap {
  src_start: u64,
  src_end: u64,
  dest_start: u64,
}
impl SourceDestMap {
  pub fn new(line: &str) -> SourceDestMap {
    let mut split_line = line.split(' ');
    let dest_start = split_line.next()
      .expect("SourceDestMap dest_start error")
      .parse::<u64>()
      .expect("SourceDestMap dest_start parse error");
    let src_start = split_line.next()
      .expect("SourceDestMap src_start error")
      .parse::<u64>()
      .expect("SourceDestMap src_start parse error");
    let size = split_line.next()
      .expect("SourceDestMap size error")
      .parse::<u64>()
      .expect("SourceDestMap size parse error");
    let src_end = src_start + size - 1; // make the end inclusive, that way range is [s..e]

    SourceDestMap { src_start, src_end, dest_start }
  }

  pub fn destination_comes_from(&self, value: u64) -> Option<u64> {
    if value >= self.dest_start && value <= self.dest_start + (self.src_end - self.src_start) {
      let diff = value - self.dest_start;
      return Some(self.src_start + diff);
    };

    None
  }

  pub fn next_destination(&self, current: u64) -> Option<u64> {
    if current >= self.src_start && current <= self.src_end {
      let diff = current - self.src_start;
      return Some(self.dest_start + diff);
    }

    None
  }
}
