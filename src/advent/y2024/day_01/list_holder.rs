
pub struct ListHolder {
  left: Vec<u32>,
  right: Vec<u32>,
}
impl ListHolder {
  pub fn new(paper_ists: &String) -> ListHolder {
    let mut left: Vec<u32> = Vec::with_capacity(paper_ists.lines().count());
    let mut right: Vec<u32> = Vec::with_capacity(paper_ists.lines().count());

    for line in paper_ists.lines() {
      let (left_value, right_value) = line.split_once("   ").expect("??");
      left.push(left_value.parse::<u32>().expect(format!("left {left_value} value was not a number").as_str()));
      right.push(right_value.parse::<u32>().expect(format!("left {left_value} value was not a number").as_str()));
    }

    left.sort_unstable();
    right.sort_unstable();

    return ListHolder { left, right }
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
}
