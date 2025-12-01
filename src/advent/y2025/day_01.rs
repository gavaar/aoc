use crate::shared::{print_solution, print_test, read_input};

#[derive(Debug)]
struct Instructions<'a> {
  pos: u32, // starts at 50
  instruction_list: Vec<&'a str>,
  visited_numbers: Vec<u32>,
}
impl<'a> Instructions<'a> {
  pub fn new(input: &'a String) -> Instructions<'a> {
    let instruction_list = input.lines().collect();
    Instructions {
      pos: 50u32,
      instruction_list,
      visited_numbers: Vec::new(),
    }
  }

  pub fn apply_instruction(&mut self, instr: &str) {
    let (dir, amount_str) = instr.split_at(1);
    let amount = amount_str.parse::<u32>().expect("received non number where number was expected");

    if dir == "L" {
      self.pos = (self.pos + (100 - (amount % 100))) % 100;
    }
    if dir == "R" {
      self.pos = (self.pos + (100 + amount)) % 100;
    }

    self.visited_numbers.push(self.pos);
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

fn amount_of_zeroes(instr: &Instructions) -> u32 {
  instr.visited_numbers.iter().filter(|num| **num == 0).count() as u32
}

pub fn run() {
  print_test();
  let test_input = read_input("day_01/test");
  let mut test_instr = build_instructions(&test_input);
  test_instr.execute_instructions();
  println!("{}", amount_of_zeroes(&test_instr));

  print_solution();
  let input = read_input("day_01/input");
  let mut instr = build_instructions(&input);
  instr.execute_instructions();
  println!("{}", amount_of_zeroes(&instr));
}
