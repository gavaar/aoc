use crate::shared::{print_solution, print_test, read_input, Color};

fn disk_map(uri: &str) -> Vec<String> {
  let input = read_input(uri);
  let mut chars = input.chars();
  let mut id = 0usize;
  let mut disk: Vec<String> = Vec::new();

  loop {
    let Some(file) = chars.next() else { break; };
    let memory = chars.next().unwrap_or('0');

    for _x in 0..file.to_digit(10).unwrap() {
      disk.push(format!("{id}"));
    }

    for _y in 0..memory.to_digit(10).unwrap() {
      disk.push(".".to_string());
    }

    id += 1;
  }

  disk
}

fn sort_disk(disk_map: &mut Vec<String>) {
  let mut dot_found = 0usize;
  let mut num_found = disk_map.len() - 1;

  loop {
    loop {
      let value = disk_map.get(dot_found).expect("should always return something");
  
      if value == &String::from('.') {
        break;
      }
  
      if dot_found == num_found {
        return;
      }
      dot_found += 1;
    }
  
    loop {
      let value = disk_map.get(num_found).expect("should always return something");

      if value != &String::from('.') {
        break;
      }
  
      if dot_found == num_found {
        return;
      }
      num_found -= 1;
    }

    disk_map.swap(dot_found, num_found);
  }
}

fn checksum(sorted_disk: &Vec<String>) -> u128 {
  sorted_disk.iter().enumerate().map(|(idx, id)| {
    if *id == String::from('.') { return 0 }
    return idx as u128 * id.parse::<u128>().unwrap();
  }).sum()
}

fn part_one(disk_map: &mut Vec<String>) {
  sort_disk(disk_map);
  let checksum = checksum(&disk_map);
  
  println!("checksum: {}", Color::Blue(checksum));
}

pub fn run() {
  print_test();
  let mut test_disk_map = disk_map("day_09/test");
  part_one(&mut test_disk_map);
  println!();
  
  print_solution();
  let mut disk_map = disk_map("day_09/input");
  part_one(&mut disk_map);
}
