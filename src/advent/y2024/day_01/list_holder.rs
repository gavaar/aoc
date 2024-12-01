use std::collections::HashMap;


pub struct ListHolder {
  left: Vec<u32>,
  right: Vec<u32>,
  similarity_map: HashMap<u32, u32>,
}
impl ListHolder {
  pub fn new(paper_ists: &String) -> ListHolder {
    let mut left: Vec<u32> = Vec::with_capacity(paper_ists.lines().count());
    let mut right: Vec<u32> = Vec::with_capacity(paper_ists.lines().count());
    let mut similarity_map: HashMap<u32, u32> = HashMap::new();

    for line in paper_ists.lines() {
      let (left_value, right_value) = line.split_once("   ").expect("??");
      let left_value_int = left_value.parse::<u32>().expect(format!("left {left_value} value was not a number").as_str());
      let right_value_int = right_value.parse::<u32>().expect(format!("left {right_value} value was not a number").as_str());

      left.push(left_value_int);
      right.push(right_value_int);

      if let Some(count) = similarity_map.get(&right_value_int) {
        similarity_map.insert(right_value_int, count + 1);
      } else {
        similarity_map.insert(right_value_int, 1);
      }
    }

    left.sort_unstable();
    right.sort_unstable();

    return ListHolder { left, right, similarity_map }
  }

  pub fn get_diffs_vector(&self) -> Vec<u32> {
    let list_len = self.left.len();
    let mut diffs_vec: Vec<u32> = Vec::with_capacity(list_len);

    for i in 0..list_len {
      let left_value = self.left[i];
      let right_value = self.right[i];
      let diff = if left_value > right_value { left_value - right_value } else { right_value - left_value };

      diffs_vec.push(diff);
    }

    diffs_vec
  }

  pub fn similarity_score(&self) -> u32 {
    self.left.iter()
      .map(|value| {
        let similarity_value = self.similarity_map.get(value).unwrap_or(&0u32);
        value * similarity_value
      }).sum()
  }
}
