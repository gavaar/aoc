use std::{collections::{HashSet, VecDeque, HashMap}, vec};

use crate::shared::{print_test, read_input, Color, print_solution, ToU8Map, report_progress};

mod direction;
use direction::Direction;
mod pos;
use pos::Pos;
mod helpers;
use helpers::draw;

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

// we won't add nodes with limitations
struct BetterQueue {
  min_overheat: usize,
  max_overheat: usize,
  heat_map: Vec<Vec<u8>>,
  seen_nodes: SeenNodes,
  priority_queue: VecDeque<Node>,
  node_in_queue_index_map: HashMap<(Pos, Direction, usize), usize>, // index where node is in queue
}
impl BetterQueue {
  fn heat_value(&self, pos: &Pos) -> Option<u32> {
    self.heat_map.get(pos.0).and_then(|r| r.get(pos.1)).and_then(|v| Some(*v as u32))
  }

  pub fn build_new_enqueue(&mut self, node: &Node, direction: &str) {
    let next_dir = node.direction.next(direction);
    let Some(new_pos) = node.pos.next(&next_dir) else { return; };
    let Some(next_heat) = self.heat_value(&new_pos) else { return; };
    let new_overheat = if direction == "forward" { node.overheat + 1 } else { 1 };
    let new_heat = node.heat_loss + next_heat;
    let mut new_path = node.path.clone();
    new_path.push(new_pos.clone());

    self.enqueue(Node {
      direction: next_dir,
      heat_loss: new_heat,
      pos: new_pos,
      overheat: new_overheat,
      path: new_path,
    });
  }

  // fn queue_move_up(&mut self, idx: usize) -> usize /* new pos */ {
  //   let mut curr_node_pos = idx;
  //   loop {
  //     let Some(curr_node) = self.priority_queue.get(curr_node_pos) else { return curr_node_pos; };
  //     let Some(next_node) = self.priority_queue.get(curr_node_pos + 1) else { return curr_node_pos; };

  //     if curr_node.heat_loss > next_node.heat_loss {
  //       self.priority_queue.swap(curr_node_pos, curr_node_pos + 1);
  //       curr_node_pos += 1;
  //     } else {
  //       return curr_node_pos;
  //     }
  //   }
  // }

  fn queue_move_down(&mut self, idx: usize) {
    let mut curr_node_idx = idx;
    loop {
      if curr_node_idx == 0 { return; };
      let Some(curr_node) = self.priority_queue.get(curr_node_idx) else { return; };
      let Some(next_node) = self.priority_queue.get(curr_node_idx - 1) else { return; };

      if curr_node.heat_loss < next_node.heat_loss {
        *self.node_in_queue_index_map.get_mut(&(curr_node.pos.clone(), curr_node.direction, curr_node.overheat)).unwrap() -= 1;
        *self.node_in_queue_index_map.get_mut(&(next_node.pos.clone(), next_node.direction, next_node.overheat)).unwrap() += 1;

        self.priority_queue.swap(curr_node_idx, curr_node_idx - 1);
        curr_node_idx -= 1;
      } else {
        return;
      }
    }
  }

  pub fn new(heat_map: Vec<Vec<u8>>, min_overheat: usize, max_overheat: usize) -> Self {
    BetterQueue {
      min_overheat,
      max_overheat,
      heat_map,
      seen_nodes: SeenNodes::default(),
      priority_queue: VecDeque::from([Node::default()]),
      node_in_queue_index_map: HashMap::new(),
    }
  }

  pub fn next(&mut self) -> Option<Node> {
    let Some(next) = self.priority_queue.pop_front() else {
      return None;
    };
    
    self.node_in_queue_index_map.remove(&(next.pos.clone(), next.direction, next.overheat));

    self.node_in_queue_index_map.values_mut().for_each(|v| {
      *v -= 1;
    });

    self.seen_nodes.see(&next);
    Some(next)
  }

  pub fn enqueue(&mut self, node: Node) {
    if self.seen_nodes.check(&node) {
      return; // we already walked it, we don't want to do nothing with it.
    }

    // find first free node
    if node.overheat < self.min_overheat {
      self.build_new_enqueue(&node, "forward");
      return;
    }

    if node.overheat >= self.max_overheat {
      for dir in ["left", "right"] {
        self.build_new_enqueue(&node, dir);
      }
      return;
    }

    let node_pos = node.pos.clone();
    let node_direction = node.direction;
    let node_overheat = node.overheat;
    match self.node_in_queue_index_map.get(&(node_pos.clone(), node_direction, node_overheat)) {
      Some(exists_in_idx) => {
        let existing_node = &mut self.priority_queue[*exists_in_idx];
        // node is smaller or equal with less overheat => we update it in the queue
        if node.heat_loss < existing_node.heat_loss {
            existing_node.direction = node.direction;
            existing_node.heat_loss = node.heat_loss;
            existing_node.path = node.path;
            existing_node.overheat = node.overheat;

            // move up the queue after updating
            self.queue_move_down(*exists_in_idx);
        }
        // else do nothing
      }
      None => {
        self.priority_queue.push_back(node);
        self.node_in_queue_index_map.insert((node_pos, node_direction, node_overheat), self.priority_queue.len() - 1);
        self.queue_move_down(self.priority_queue.len() - 1);
      }
    }
  }
}

struct SeenNodes(HashSet<(Pos, Direction, usize)>);
impl Default for SeenNodes {
  fn default() -> Self {
    SeenNodes(HashSet::new())
  }
}
impl SeenNodes {
  fn see(&mut self, node: &Node) {
    self.0.insert((node.pos.clone(), node.direction, node.overheat));
  }

  fn check(&self, node: &Node) -> bool {
    self.0.get(&(node.pos.clone(), node.direction, node.overheat)).is_some()
  }

  fn len(&self) -> usize {
    self.0.len()
  }
}

fn find_path(map: &String, min_overheat: usize, max_overheat: usize) {
  let heat_loss_map = map.to_u8_map();
  let heat_loss_last_row_idx = heat_loss_map.len() - 1;
  let heat_loss_last_col_idx = heat_loss_map[0].len() - 1;
  let total = heat_loss_map.iter().flatten().count() * 24;

  let mut queue = BetterQueue::new(heat_loss_map, min_overheat, max_overheat);
  let mut last_node = Node::default();

  while let Some(next_node) = queue.next() {
    if next_node.pos == Pos(heat_loss_last_row_idx, heat_loss_last_col_idx) {
      last_node = next_node;
      break;
    }

    report_progress(queue.seen_nodes.len(), total);

    for dir in ["forward", "left", "right"] {
      queue.build_new_enqueue(&next_node, dir);
    }
  }

  draw(&map.to_u8_map(), &last_node.path);

  println!("The shortest path sums: {}", Color::Red(last_node.heat_loss));
}

pub fn run() {
  print_test();
  let map = read_input("day_17/test");
  println!("--> {}", Color::Blue("part one"));
  find_path(&map, 1, 3);
  println!("--> {}", Color::Blue("part two"));
  let map_2 = read_input("day_17/test_2");
  find_path(&map, 4, 10);
  find_path(&map_2, 4, 10);
  
  println!();
  
  print_solution();
  let map = read_input("day_17/input");
  println!("--> {}", Color::Blue("part one"));
  find_path(&map, 1, 3);
  println!("--> {}", Color::Blue("part two"));
  find_path(&map, 4, 10);
}
