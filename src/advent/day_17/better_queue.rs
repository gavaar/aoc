use std::collections::{VecDeque, HashMap};

use super::{seen_nodes::SeenNodes, node::Node, pos::Pos, direction::Direction};


// we won't add nodes with limitations
pub struct BetterQueue {
  min_overheat: usize,
  max_overheat: usize,
  heat_map: Vec<Vec<u8>>,
  priority_queue: VecDeque<Node>,
  node_in_queue_index_map: HashMap<(Pos, Direction, usize), usize>, // index where node is in queue
  pub seen_nodes: SeenNodes,
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
