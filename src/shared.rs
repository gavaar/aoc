use std::fs;

pub fn read_input(uri: &str) -> String {
  fs::read_to_string(uri).expect("error reading file")
}
