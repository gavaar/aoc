mod robot;

use core::panic;

use crate::shared::{print_solution, print_test, read_input, Color};

use robot::Robot;

const TEST_MAP_SIZE: (usize, usize) = (11, 7);
const MAP_SIZE: (usize, usize) = (101, 103);

fn get_quadrants(robots: &Vec<Robot>) -> [usize; 4] {
  let mut quadrants = [0; 4];
  
  robots.iter().for_each(|robot| {
    let frontier_x = (robot.teleport_x - 1) / 2;
    let frontier_y = (robot.teleport_y - 1) / 2;

    if robot.x() == frontier_x || robot.y() == frontier_y {
      return;
    }

    let quadrant: usize = 
      // top-left quadrant
      if robot.x() < frontier_x && robot.y() < frontier_y { 0 }
      // top-right quadrant
      else if robot.x() > frontier_x && robot.y() < frontier_y { 1 }
      // bot-left quadrant
      else if robot.x() < frontier_x && robot.y() > frontier_y { 2 }
      // bot-right quadrant
      else if robot.x() > frontier_x && robot.y() > frontier_y { 3 }
      else { panic!("should not happen") };

    *quadrants.get_mut(quadrant).unwrap() += 1;
  });

  quadrants
}

fn build_robots(uri: &str, size: &(usize, usize)) -> Vec<Robot> {
  let input = read_input(uri);
  let robots = input.lines().map(|line| Robot::new(line, size)).collect();
  robots
}

fn part_one(robots: &mut Vec<Robot>) {
  robots.iter_mut().for_each(|robot| robot.walk(100));
  let quadrants = get_quadrants(robots);
  let mult = quadrants.iter().fold(1, |acc, x| acc * x);
  println!("The safety factor is {}", Color::Blue(mult));
}

pub fn run() {
  print_test();
  let mut test_robots = build_robots("day_14/test", &TEST_MAP_SIZE);
  part_one(&mut test_robots);
  println!();

  print_solution();
  let mut robots = build_robots("day_14/input", &MAP_SIZE);
  part_one(&mut robots);
}
