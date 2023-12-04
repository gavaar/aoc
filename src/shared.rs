use std::fs;

pub fn read_input(uri: &str) -> String {
  let url = String::from("src/advent/") + uri;
  fs::read_to_string(url).expect("error reading file")
}
