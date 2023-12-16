
pub struct Boxes<'a>(Vec<Vec<(&'a str, u8)>>);
impl<'a> Default for Boxes<'a> {
  fn default() -> Self {
    Boxes(vec![Vec::new(); 256])
  }
}
impl<'a> Boxes<'a> {
  pub fn hash_algorithm(seq: &str) -> u32 {
    seq.chars().fold(0u32, |acc, curr_char| {
      let ascii =  curr_char as u8 as u32;
      ((acc + ascii) * 17) % 256u32
    })
  }

  fn sequence_operation(seq: &str) -> char {
    seq.chars().rev()
      .find(|c| c == &'=' || c == &'-')
      .expect("did not find operation")
  }

  pub fn operate(&mut self, sequence: &'a str) {
    let operation = Boxes::sequence_operation(sequence);
    let (label, focal_length) = sequence
      .split_once(operation)
      .expect("Boxes::operate - something broke splitting");
    let box_id = Boxes::hash_algorithm(label) as usize;

    match operation {
      '=' => self.replace(box_id, label, focal_length),
      '-' => self.remove(box_id, label),
      _ => unreachable!(),
    }
  }

  pub fn focusing_power(&self) -> u32 {
    self.0.iter().enumerate().map(|(box_number, single_box)| {
      /*
      - One plus the box number of the lens in question.
      - The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
      - The focal length of the lens.
       */
      single_box.iter().enumerate().map(|(slot, box_value)| {
        (1u32 + box_number as u32) * (1u32 + slot as u32) * box_value.1 as u32
      }).sum::<u32>()
    }).sum()
  }

  fn lens_index(&self, box_id: usize, label: &'a str) -> Option<usize> {
    self.0[box_id]
      .iter()
      .position(|labeled_lens| {
        labeled_lens.0 == label
      })
  }

  fn replace(&mut self, box_id: usize, label: &'a str, focal_len: &'a str) {
    let lens_index = self.lens_index(box_id, label);
    let focal_len_value = focal_len.parse().expect("Boxes::replace - focal_len parse error");

    if let Some(found_lens_index) = lens_index {
      self.0[box_id][found_lens_index].1 = focal_len_value;
    } else {
      self.0[box_id].push((label, focal_len_value))
    }
  }

  fn remove(&mut self, box_id: usize, label: &'a str) {
    let lens_index = self.lens_index(box_id, label);
    
    if let Some(found_lens_index) = lens_index {
      self.0[box_id].remove(found_lens_index);
    }
  }
}
