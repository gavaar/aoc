use crate::shared::{print_test, print_solution, read_input, Color};

mod map;
use map::Map;

use num::Integer;

fn part_one(map: &mut Map) {
  loop {
    if map.current_node == "ZZZ" {
      break;
    }

    map.next();
  }

  println!("it took {} steps for map to reach {}", Color::Red(map.steps_taken), Color::Red(map.current_node));
}

fn part_two(paper: &String) {
  let mut lines = paper.lines();
  lines.nth(1); // skip two lines
  let mut vec_map: Vec<Map> = Vec::new();

  for line in lines {
    let id = line.split_at(3).0;
    if id.ends_with('A') {
      let map = Map::new(paper, id);
      vec_map.push(map);
    }
  }

  for map in vec_map.iter_mut() {
    loop {
      if map.current_node.ends_with('Z') {
        break;
      }

      map.next();
    }

    println!("in {} steps, map reached {}", Color::Red(map.steps_taken), Color::Red(map.current_node));
  }

  let lcm = vec_map.iter().fold(1, |acc, b| {
    let new = acc.lcm(&b.steps_taken);
    new
  });

  println!("the lowest common multiple is {}", Color::Red(lcm));
}

pub fn run() {
  print_test();
  let test_input_1 = read_input("day_08/test_1");
  let test_input_2 = read_input("day_08/test_2");
  let mut map_test_1 = Map::new(&test_input_1, "AAA");
  let mut map_test_2 = Map::new(&test_input_2, "AAA");
  
  part_one(&mut map_test_1);
  part_one(&mut map_test_2);
  println!();
  
  let test_input_3 = read_input("day_08/test_3");
  part_two(&test_input_3);
  println!();
  
  print_solution();
  let input = read_input("day_08/input");
  let mut map = Map::new(&input, "AAA");
  part_one(&mut map);

  let input_2 = read_input("day_08/input");
  part_two(&input_2);
}
