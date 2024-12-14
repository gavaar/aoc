use crate::shared::{print_solution, print_test, read_input, Color};

fn disk_spaces(uri: &str) -> Vec<(String, usize)> {
  let input = read_input(uri);
  let mut id = 0usize;
  let mut chars = input.chars();
  let mut disk: Vec<(String, usize)> = Vec::new();

  loop {
    let Some(file) = chars.next() else { break; };
    let memory = chars.next().unwrap_or('0');

    disk.push((id.to_string(), file.to_digit(10).unwrap() as usize));
    disk.push(('.'.to_string(), memory.to_digit(10).unwrap() as usize));

    id += 1;
  }

  disk
}

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

fn sort_spaces(disk: &mut Vec<(String, usize)>) {
  let disk_len = disk.len().to_owned();
  let mut dot_found = 0usize;
  let mut num_found = disk_len - 1;

  'outer: loop {
    let mut num_value;
    loop {
      num_value = disk.get(num_found).expect("always exists");

      if num_value.0 != '.'.to_string() {
        break;
      }

      if num_found == 0 {
        return;
      }

      num_found -= 1;
    }

    let mut dot_value;
    loop {
      dot_value = disk.get(dot_found).expect("every time works").clone();

      if dot_value.0 == '.'.to_string() && dot_value.1 >= num_value.1 {
        break;
      }

      if num_found == 0 {
        return;
      }

      if dot_found >= num_found {
        dot_found = 0;
        num_found -= 1;
        continue 'outer;
      }

      dot_found += 1;
    }

    let popped_element = disk.remove(num_found);
    let popped_size = popped_element.1;
    disk.insert(dot_found, popped_element);
    dot_found += 1;

    // the dot we found, has to remove it's space
    disk.get_mut(dot_found).unwrap().1 -= popped_size;
    disk.insert(num_found, ('.'.to_string(), popped_size));
    
    dot_found = 0;
  }
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

fn spaces_into_id_vec(spaces: &Vec<(String, usize)>) -> Vec<String> {
  let mut result = Vec::new();

  spaces.iter().for_each(|(id, amount)| {
    for _x in 0..*amount {
      result.push(id.clone());
    }
  });

  result
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

fn part_two(disk_spaces: &mut Vec<(String, usize)>) {
  sort_spaces(disk_spaces);
  let final_disk = spaces_into_id_vec(&disk_spaces);
  let checksum = checksum(&final_disk);
  
  println!("checksum: {}", Color::Blue(checksum));
}

pub fn run() {
  print_test();
  let mut test_disk_map = disk_map("day_09/test");
  part_one(&mut test_disk_map);
  let mut test_disk_spaces = disk_spaces("day_09/test");
  part_two(&mut test_disk_spaces);
  println!();
  
  print_solution();
  let mut disk_map = disk_map("day_09/input");
  part_one(&mut disk_map);
  let mut disk_spaces = disk_spaces("day_09/input");
  part_two(&mut disk_spaces);
}
