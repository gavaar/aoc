use crate::{components::{Map2D, Map2DDirection}, shared::{Color, read_input}};

struct PrintingDpt {
  grid_map: Vec<Vec<char>>,
}
impl PrintingDpt {
  pub fn new(input: &String) -> PrintingDpt {
    PrintingDpt {
      grid_map: PrintingDpt::build_from_char_text(input),
    }
  }
}
impl Map2D for PrintingDpt {
  fn grid_map(&self) -> &Vec<Vec<char>> {
    &self.grid_map
  }
}

fn find_movable_rolls(map: &PrintingDpt) {
  let mut can_be_moved = 0;

  for row in 0..map.rows() {
    for col in 0..map.cols() {
      let current = (row, col);
      if map.get(&current) == Some(&'.') { continue }

      let roll_count = Map2DDirection::get_all_dirs()
        .iter()
        .map(|dir| map.get_at_dir(&current, &dir).unwrap_or((0,0,'.')))
        .filter(|(_r, _c, value)| value == &'@')
        .count();
      if roll_count < 4 {
        can_be_moved += 1;
      }
    }
  }
  
  println!("Rolls that can be moved: {}", Color::Green(can_be_moved));
}

pub fn run() {
  let test_input = read_input("day_04/test");
  let test_map = PrintingDpt::new(&test_input);
  find_movable_rolls(&test_map);

  println!();
  let input = read_input("day_04/input");
  let map = PrintingDpt::new(&input);
  find_movable_rolls(&map);
}
