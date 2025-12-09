use std::collections::{HashMap, HashSet};

use num::integer::sqrt;

use crate::shared::{Color, print_solution, print_test, read_input};

fn get_junctions_key(a: &(usize, usize, usize), b: &(usize, usize, usize)) -> ((usize, usize, usize), (usize, usize, usize)) {
  let a_sum = a.0 + a.1 + a.2;
  let b_sum = b.0 + b.1 + b.2;
  if a_sum <= b_sum { (*a, *b) } else { (*b, *a) }
}

fn distance_3d(a: &(usize, usize, usize), b: &(usize, usize, usize)) -> u128 {
  sqrt(
    (a.0 as isize - b.0 as isize).pow(2) +
    (a.1 as isize - b.1 as isize).pow(2) +
    (a.2 as isize - b.2 as isize).pow(2)
  ) as u128
}

#[derive(Debug)]
struct UndergroundSpace {
  junction_boxes: HashSet<(usize, usize, usize)>,
  distance_map: HashMap<((usize,usize,usize), (usize,usize,usize)), u128>,
}
impl UndergroundSpace {
  pub fn new(uri: &str) -> Self {
    let input = read_input(uri);
    let mut junction_boxes = HashSet::new();
    let mut distance_map = HashMap::new();

    for line in input.lines() {
      let mut line_iter = line.split(",").into_iter();
      let x = line_iter.next().expect("should exist").parse::<usize>().expect("should be number");
      let y = line_iter.next().expect("should exist").parse::<usize>().expect("should be number");
      let z = line_iter.next().expect("should exist").parse::<usize>().expect("should be number");

      junction_boxes.insert((x, y, z));
    }

    for junction_1 in junction_boxes.iter() {
      for junction_2 in junction_boxes.iter() {
        if junction_1 == junction_2 { continue; }
        let key = get_junctions_key(junction_1, junction_2);
        if distance_map.contains_key(&key) { continue; }

        distance_map.insert(
        key,
        distance_3d(junction_1, junction_2)
        );
      }
    }

    UndergroundSpace {
      junction_boxes,
      distance_map,
    }
  }

  // (<circuit set>, <points>)
  pub fn make_x_circuits(&self, x: usize) -> (HashSet::<Vec<(usize, usize, usize)>>, ((usize, usize, usize), (usize, usize, usize))) {
    let mut starting_circuits = HashSet::<Vec<(usize, usize, usize)>>::from_iter(
      self.junction_boxes.iter().map(|jb| {
        let mut set = Vec::<(usize, usize, usize)>::new();
        set.push(*jb);
        set
      })
    );

    let mut distances: Vec<(&((usize,usize,usize), (usize,usize,usize)), &u128)> = self.distance_map.iter().collect();
    distances.sort_by(|(_, distance_a), (_, distance_b)| distance_a.cmp(distance_b));
    let (sorted_distances, _) = if x > 0 { distances.split_at(x) } else { distances.split_at(distances.len()) };

    let mut last_two_connections = ((0,0,0),(0,0,0));

    for ((jb_a, jb_b), _distance) in sorted_distances {
      let mut circuit_a = starting_circuits
        .iter()
        .find(|circuit| circuit.contains(jb_a))
        .expect("has to be found")
        .clone();
      let mut circuit_b = starting_circuits
        .iter()
        .find(|circuit| circuit.contains(jb_b))
        .expect("has to be found")
        .clone();

      let same_circuit = circuit_a.contains(jb_b) && circuit_b.contains(jb_a);
      if same_circuit { continue; }
      
      starting_circuits.remove(&circuit_a);
      starting_circuits.remove(&circuit_b);
      circuit_a.append(&mut circuit_b);
      starting_circuits.insert(circuit_a);

      if x == 0 && starting_circuits.len() == 1 {
        last_two_connections = (*jb_a, *jb_b);
        break;
      }
    }

    (starting_circuits, last_two_connections)
  }
}

fn part_one(uri: &str, reps: usize) {
  let url = format!("day_08/{uri}");
  let ug_space = UndergroundSpace::new(url.as_str());
  let (circuits, _) = ug_space.make_x_circuits(reps);
  let mut sorted_circuits: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
  sorted_circuits.sort_by(|a,b| b.cmp(a));
  let biggest_three: u128 = sorted_circuits.split_at(3).0.iter().map(|v| *v as u128).product();

  println!("sizes: {}", Color::Green(biggest_three));
}

fn part_two(uri: &str) {
  let url = format!("day_08/{uri}");
  let ug_space = UndergroundSpace::new(url.as_str());
  let (_circuits, last_two) = ug_space.make_x_circuits(0);

  println!("last two: {}", Color::Green(last_two.0.0 * last_two.1.0));
}

pub fn run() {
  print_test();
  part_one("test", 10);
  part_two("test");

  println!();

  print_solution();
  part_one("input", 1000);
  part_two("input");
}
