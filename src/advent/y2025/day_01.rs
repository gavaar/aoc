use crate::shared::{Color, print_solution, print_test, read_input};

#[derive(Debug)]
struct Instructions<'a> {
  pos: u32, // starts at 50
  instruction_list: Vec<&'a str>,
  visited_numbers: Vec<u32>,
  pub zero_passings: u32,  
}
impl<'a> Instructions<'a> {
  pub fn new(input: &'a String) -> Instructions<'a> {
    let instruction_list = input.lines().collect();
    Instructions {
      pos: 50u32,
      instruction_list,
      visited_numbers: Vec::new(),
      zero_passings: 0u32,
    }
  }

  pub fn apply_instruction(&mut self, instr: &str) {
    let (dir, amount_str) = instr.split_at(1);
    let amount = amount_str.parse::<u32>().expect("received non number where number was expected");
    let leftover_amount = amount % 100;
    let mut circles_around_zero = (amount - leftover_amount) / 100;
    let safe_starting_value = self.pos + 100; // eg we are at 21, we set 121 so we can substract up to 99
    let final_value = if dir == "L" { safe_starting_value - leftover_amount } else { safe_starting_value + leftover_amount };
    if safe_starting_value > 100 && final_value <= 100 || final_value >= 200 { circles_around_zero += 1 }; // it went through zero

    let new_value = final_value % 100;

    self.pos = new_value;
    self.visited_numbers.push(new_value);
    self.zero_passings += circles_around_zero;
  }

  pub fn execute_instructions(&mut self) {
    let iter_list = self.instruction_list.clone();
    iter_list.iter().for_each(|instruction| {
      self.apply_instruction(instruction);
    });
  }
}

fn build_instructions<'a>(input: &'a String) -> Instructions<'a> {
  let instr = Instructions::new(input);
  instr
}

fn amount_of_zeroes(instr: &Instructions) -> Color<u32> {
  Color::Blue(instr.visited_numbers.iter().filter(|num| **num == 0).count() as u32)
}

pub fn run() {
  print_test();
  let test_input = read_input("day_01/test");
  let mut test_instr = build_instructions(&test_input);
  test_instr.execute_instructions();
  println!("amount of zero landing: {}", amount_of_zeroes(&test_instr));
  println!("amount of zero passings: {}", Color::Blue(test_instr.zero_passings));

  println!();

  print_solution();
  let input = read_input("day_01/input");
  let mut instr = build_instructions(&input);
  instr.execute_instructions();
  println!("amount of zero landing: {}", amount_of_zeroes(&instr));
  println!("amount of zero passings: {}", Color::Blue(instr.zero_passings));
}
