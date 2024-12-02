#[derive(Debug)]
pub struct Report {
  files: Vec<u8>,
  problem_dampener: bool,
}
impl Report {
  pub fn new(report_text: &str) -> Report {
    let files: Vec<u8> = report_text.split(" ").map(|value| value.parse::<u8>().expect("??")).collect();
    Report { files, problem_dampener: false }
  }

  pub fn activate_dampener(&mut self) {
    self.problem_dampener = true;
  }

  pub fn is_safe(&mut self) -> bool {
    let mut direction: Option<bool> = None;
    let mut skipped = false;

    for i in 1..self.files.len() {
      let curr = self.files.get(i).expect(format!("no file found: {i}").as_str());
      let prev = self.files.get(i - 1).expect(format!("no file found: {}", i - 1).as_str());

      if direction == None { direction = direction_calc(&curr, &prev) }
      let is_safe = are_safe_values(&direction, *curr as i16, *prev as i16);

      if !is_safe {
        if !self.problem_dampener || skipped {
          return false
        }

        skipped = true;

        let prev_prev = if i == 1 { None } else { self.files.get(i - 2) };
        let next = self.files.get(i + 1);

        let next_curr_safe = if next.is_none() {
          // over last case
          true
        } else {
          let mut temp_dir = direction.to_owned();
          if i == 1 || i == 2 {
            temp_dir = direction_calc(next.unwrap(), curr);
          }
          are_safe_values(&temp_dir, *next.unwrap() as i16, *curr as i16)
        };
        let curr_prev_prev_safe = if prev_prev.is_none() {
          // below first case
          true
        } else {
          let mut temp_dir = direction.to_owned();
          if i == 1 || i == 2 {
            temp_dir = direction_calc(curr, prev_prev.unwrap());
          }
          are_safe_values(&temp_dir, *curr as i16, *prev_prev.unwrap() as i16)
        };
        
        let next_prev_safe = if next.is_none() {
          // over last case
          true
        } else {
          let mut temp_dir = direction.to_owned();
          if i == 1 || i == 2 {
            temp_dir = direction_calc(next.unwrap(), prev);
          }
          are_safe_values(&temp_dir, *next.unwrap() as i16, *prev as i16)
        };
        let prev_prev_prev_safe = if prev_prev.is_none() {
          // before first case
          true
        } else {
          let mut temp_dir = direction.to_owned();
          if i == 1 || i == 2 {
            temp_dir = direction_calc(prev, prev_prev.unwrap());
          }
          are_safe_values(&temp_dir, *prev as i16, *prev_prev.unwrap() as i16)
        };

        if next_curr_safe && curr_prev_prev_safe {
          direction = None;
          continue;
        }

        if next_prev_safe && prev_prev_prev_safe {
          direction = None;
          // swap so prev becomes curr and curr is destroyed in history
          self.files.swap(i, i - 1);
          continue;
        }

        return false;
      }
    }
    
    return true;
  }
}

fn direction_calc(file: &u8, prev_number: &u8) -> Option<bool> {
  if file > prev_number { Some(true) } else { Some(false) }
}

fn are_safe_values(direction: &Option<bool>, file: i16, prev: i16) -> bool {
  let is_positive_direction = &Some(true) == direction;

  let prev_num_diff = file - prev;

  if is_positive_direction {
    prev_num_diff >= 1 && prev_num_diff <= 3
  } else {
    prev_num_diff <= -1 && prev_num_diff >= -3
  }
}
