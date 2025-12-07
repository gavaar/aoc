mod robot;

// use core::panic;
use std::{collections::HashSet,
  // iter
};

use crate::{components::{Drawable, Map2D}, shared::{print_solution, print_test, read_input, Color}};

use robot::Robot;

// const TEST_MAP_SIZE: (usize, usize) = (11, 7);
const MAP_SIZE: (usize, usize) = (101, 103);

struct Map {
  grid: Vec<Vec<char>>,
}
impl Map {
  fn new(size: &(usize, usize), robots: &Vec<Robot>) -> Map {
    let mut grid = vec![vec!['.'; size.0]; size.1];
    robots.iter().for_each(|rb| grid[rb.y()][rb.x()] = '*');
    Map { grid }
  }
}
impl Map2D<char> for Map {
  fn grid_map(&self) -> &Vec<Vec<char>> {
    &self.grid
  }
}
impl Drawable<char> for Map {
  fn color_override(&self, value: impl std::fmt::Display, _pos: &(usize, usize)) -> Color<impl std::fmt::Display> {
    if value.to_string().as_str() != "." {
      return Color::Green(value);
    }

    Color::Default(value)
  }
}

// fn get_quadrants(robots: &Vec<Robot>) -> [usize; 4] {
//   let mut quadrants = [0; 4];
  
//   robots.iter().for_each(|robot| {
//     let frontier_x = (robot.teleport_x - 1) / 2;
//     let frontier_y = (robot.teleport_y - 1) / 2;

//     if robot.x() == frontier_x || robot.y() == frontier_y {
//       return;
//     }

//     let quadrant: usize = 
//       // top-left quadrant
//       if robot.x() < frontier_x && robot.y() < frontier_y { 0 }
//       // top-right quadrant
//       else if robot.x() > frontier_x && robot.y() < frontier_y { 1 }
//       // bot-left quadrant
//       else if robot.x() < frontier_x && robot.y() > frontier_y { 2 }
//       // bot-right quadrant
//       else if robot.x() > frontier_x && robot.y() > frontier_y { 3 }
//       else { panic!("should not happen") };

//     *quadrants.get_mut(quadrant).unwrap() += 1;
//   });

//   quadrants
// }

fn build_robots(uri: &str, size: &(usize, usize)) -> Vec<Robot> {
  let input = read_input(uri);
  let robots = input.lines().map(|line| Robot::new(line, size)).collect();
  robots
}

// fn part_one(robots: &mut Vec<Robot>) {
//   robots.iter_mut().for_each(|robot| robot.walk(100));
//   let quadrants = get_quadrants(robots);
//   let mult = quadrants.iter().fold(1, |acc, x| acc * x);
//   println!("The safety factor is {}", Color::Blue(mult));
// }

fn part_two(robots: &mut Vec<Robot>, size: &(usize, usize)) {
  let middle = (size.0 - 1) / 2;
  let mut seconds = 0;
  let mut robots_hash: HashSet<(usize, usize)> = HashSet::new();
  let mut iterations = 0;

  let vertical_limit = 1;
  let horiz_limit = 3;
  let total_iterations = 2;

  while robots_hash.len() < size.1 / 2 || (iterations < total_iterations) {
    robots.iter_mut().for_each(|rb| rb.walk(1));
    seconds += 1;

    robots_hash = robots.iter().fold(HashSet::new(), |mut hs, rb| {
      let horiz_half = (horiz_limit - 1) / 2;
      if rb.x() <= middle + horiz_half &&
         rb.x() >= middle - horiz_half 
        //  rb.y() >= size.1 - vertical_limit
      {
        hs.insert((rb.x(), rb.y()));
      }
      return hs;
    });

    if robots_hash.len() >= (vertical_limit * horiz_limit) {
      iterations += 1;

      let map = Map::new(size, robots);
      map.draw();
      println!("{seconds} - {iterations}");
    }
  }
}

pub fn run() {
  print_test();
  // let mut test_robots = build_robots("day_14/test", &TEST_MAP_SIZE);
  // part_one(&mut test_robots);
  // let mut test_robots = build_robots("day_14/test", &TEST_MAP_SIZE);
  // part_two(&mut test_robots, &TEST_MAP_SIZE);
  println!();
  
  print_solution();
  // let mut robots = build_robots("day_14/input", &MAP_SIZE);
  // part_one(&mut robots);
  let mut robots = build_robots("day_14/input", &MAP_SIZE);
  part_two(&mut robots, &MAP_SIZE);
}
