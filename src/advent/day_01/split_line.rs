const NUMBERS: [(&'static str, char); 9] = [
  ("one", '1'),
  ("two", '2'),
  ("three", '3'),
  ("four", '4'),
  ("five", '5'),
  ("six", '6'),
  ("seven", '7'),
  ("eight", '8'),
  ("nine", '9'),
];

pub fn split_line(line: &str, read_text: bool) -> (char, char) {
  let mut ltr_nums = String::new();
  let mut rtl_nums = String::new();
  let mut ltr_str = String::new();
  let mut rtl_str = String::new();

  for i in 0..line.len() {
    let last = line.len() - i - 1;
    let curr_ltr_char = line.chars().nth(i).expect("woopsie on ltr");
    let curr_rtl_char = line.chars().nth(last).expect("woopsie on rtl");

    if let Some(_) = curr_ltr_char.to_digit(10) {
      ltr_nums.push(curr_ltr_char);
      ltr_str = String::new();
    } else {
      ltr_str.push(curr_ltr_char);
    }

    if let Some(_) = curr_rtl_char.to_digit(10) {
      rtl_nums.push(curr_rtl_char);
      rtl_str = String::new();
    } else {
      rtl_str = format!("{}{}", curr_rtl_char, rtl_str);
    }

    if read_text {
      if ltr_str.len() > 2 {
        for subs in NUMBERS {
          if ltr_str.contains(subs.0) {
            ltr_nums.push(subs.1);
            break;
          }
        }
      }

      if rtl_str.len() > 2 {
        for subs in NUMBERS {
          if rtl_str.contains(subs.0) {
            rtl_nums.push(subs.1);
            break;
          }
        }
      }
    }
  }

  (ltr_nums.chars().nth(0).unwrap(), rtl_nums.chars().nth(0).unwrap())
}
