use crate::shared::{print_test, print_solution, read_input, Color};

mod dish;
use dish::Dish;

fn parse_input(input: &String) -> Dish {
  Dish::new(input)
}

fn part_one(dish: &mut Dish) {
  dish.tilt_north();

  println!("val after tilt: {}", Color::Red(dish.value()));
}

pub fn run() {
  print_test();
  println!("-- p1");
  let mut dish = parse_input(&read_input("day_14/test"));
  part_one(&mut dish);
  
  println!();
  
  print_solution();
  println!("-- p1");
  let mut dish = parse_input(&read_input("day_14/input"));
  part_one(&mut dish);

}
