use std::{collections::{HashSet, HashMap}, vec};

use crate::shared::{print_test, read_input, Color, print_solution, ToU8Map, report_progress};

mod direction;
use direction::Direction;
mod pos;
use pos::Pos;

fn draw(heat_map: &Vec<Vec<u8>>, path: &Vec<Pos>) {
  let mut str_vec: Vec<Vec<String>> = heat_map.iter().map(|row| row.iter().map(|heat_loss| heat_loss.to_string()).collect()).collect();

  for pos in path {
    let original_value = str_vec[pos.0][pos.1].to_owned();
    str_vec[pos.0][pos.1] = Color::Green(original_value).to_string();
  }

  let mut drawing = String::new();
  for row in str_vec {
    let mut line = String::new();
    for col in row {
      line.push_str(col.as_str());
    }
    line.push('\n');
    drawing.push_str(line.as_str());
  }

  println!();
  println!("{drawing}");
}

#[derive(Debug)]
struct Node {
  pos: Pos,
  direction: Direction,
  overheat: usize,
  heat_loss: u32,
  path: Vec<Pos>,
}
impl Default for Node {
  fn default() -> Self {
    Node {
      pos: Pos(0, 0),
      direction: Direction::Right,
      overheat: 0,
      heat_loss: 0,
      path: vec![Pos(0, 0)],
    }
  }
}

struct Queue {
  priority_queue: HashMap<u32, Vec<(Pos, usize, Direction)>>,
  priority_queue_idx: u32,
  // (pos + overheat)
  hash: HashMap<(Pos, usize, Direction), Node>,
}
impl Default for Queue {
  fn default() -> Self {
    Queue {
      priority_queue: HashMap::new(),
      priority_queue_idx: 0,
      hash: HashMap::new(),
    }
  }
}
impl Queue {
  pub fn enqueue(&mut self, pos: Pos, node: Node) {
    let current_node_in_pos = self.hash.get(&(pos.clone(), node.overheat, node.direction));

    // if node is not in the list, or is but with more heat_loss or more overheat
    if current_node_in_pos.is_none() ||
      node.heat_loss < current_node_in_pos.unwrap().heat_loss {
      if !self.priority_queue.contains_key(&node.heat_loss) {
        self.priority_queue.insert(node.heat_loss, Vec::new());
      }

      self.priority_queue.get_mut(&node.heat_loss).unwrap().push((pos.clone(), node.overheat, node.direction));
      let res = self.hash.insert((pos, node.overheat, node.direction), node);
      if res.is_some() {
        println!("REMOVED {:?} from", res.unwrap());
      }
    }
  }

  pub fn next(&mut self) -> Option<Node> {
    loop {
      if self.priority_queue.len() == 0 {
        return None;
      }

      let next_priority_queue = self.priority_queue.get_mut(&self.priority_queue_idx);

      if let Some(pos_queue) = next_priority_queue {
        let next_pos = pos_queue.pop().expect("this should not fail");

        if pos_queue.len() == 0 {
          self.priority_queue.remove(&self.priority_queue_idx);
        }

        return self.hash.remove(&next_pos);
      }

      self.priority_queue_idx += 1;
    }
  }
}

fn part_one(map: &String) {
  let heat_loss_map = map.to_u8_map();
  let mut queue = Queue::default();
  queue.enqueue(Pos(0, 0), Node::default());

  let mut seen_nodes: HashSet<Pos> = HashSet::new();
  let mut last_node = Node::default();

  let total = heat_loss_map.iter().flatten().count();

  loop {
    let Some(current_node) = queue.next() else {
      println!("THIS BROKE");
      unreachable!();
    };
    report_progress(seen_nodes.len(), total);
    // update node as seen
    seen_nodes.insert(current_node.pos.clone());

    if current_node.pos == Pos(heat_loss_map.len() - 1, heat_loss_map[0].len() - 1) {
      last_node = current_node;
      break;
    };

    // check three directions
    for direction in ["right", "forward", "left"] {
      let next_direction = current_node.direction.next(direction);

      // if there's a next pos, and is not already seen
      if let Some(next_pos) = current_node.pos.next(&next_direction) {
        // if seen_nodes.contains(&next_pos) { continue; };
        
        // and we also have a value in this new pos
        if let Some(next_heat_loss) = heat_loss_map.get(next_pos.0).and_then(|r| r.get(next_pos.1)) {
          let overheat = if direction == "forward" { current_node.overheat + 1 } else { 1 };

          if overheat <= 3 {
            let new_heat_loss =  current_node.heat_loss + *next_heat_loss as u32;
            let mut path = current_node.path.clone();
            path.push(next_pos.clone());

            queue.enqueue(next_pos.clone(), Node {
              path,
              overheat,
              heat_loss: new_heat_loss,
              direction: next_direction,
              pos: next_pos,
            });
          }
        }
      };
    }
  };

  draw(&heat_loss_map, &last_node.path);

  println!("The shortest path sums: {}", Color::Red(last_node.heat_loss));
}

pub fn run() {
  print_test();
  let map = read_input("day_17/test");
  part_one(&map);

  println!();

  print_solution();
  let map = read_input("day_17/input");
  part_one(&map);
}
