use crate::shared::{print_test, print_solution, read_input, Color};

mod map;
use map::Map;

fn part_one(map: &mut Map) {
  loop {
    if map.current_node == "ZZZ" {
      break;
    }

    map.next();
  }

  println!("it took {} steps for map to reach {}", Color::Red(map.steps_taken), Color::Red(map.current_node));
}

pub fn run() {
  let test_input_1 = read_input("day_08/test_1");
  let test_input_2 = read_input("day_08/test_2");
  let input = read_input("day_08/input");
  let mut map_test_1 = Map::new(&test_input_1);
  let mut map_test_2 = Map::new(&test_input_2);
  let mut map = Map::new(&input);

  print_test();
  part_one(&mut map_test_1);
  part_one(&mut map_test_2);
  println!();
  
  print_solution();
  part_one(&mut map);
}
