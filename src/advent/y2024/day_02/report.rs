#[derive(Debug)]
pub struct Report {
  files: Vec<u8>,
}
impl Report {
  pub fn new(report_text: &str) -> Report {
    let files: Vec<u8> = report_text.split(" ").map(|value| value.parse::<u8>().expect("??")).collect();
    Report { files }
  }

  pub fn is_safe(&self) -> bool {
    // either no direction yet, positive or negative
    let mut direction: Option<bool> = None;
    // prev number
    let mut prev_number: Option<&u8> = None;
    let mut files = self.files.iter();

    while let Some(file) = files.next() {
      // first one is skipped and set as prev_number
      if prev_number == None {
        prev_number = Some(file);
        continue;
      }
      
      if direction == None {
        direction = if *file > *prev_number.unwrap() { Some(true) } else { Some(false) };
      }

      let is_positive_direction = Some(true) == direction;
      let prev_num_diff = *file as i16 - *prev_number.unwrap() as i16;
      prev_number = Some(file);

      if is_positive_direction && (prev_num_diff < 1 || prev_num_diff > 3) {
        return false;
      }
      if !is_positive_direction && (prev_num_diff > -1 || prev_num_diff < -3) {
        return false;
      }
    }
    
    return true;
  }
}
