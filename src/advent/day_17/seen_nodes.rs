use std::collections::HashSet;
use super::{pos::Pos, direction::Direction, node::Node};

pub struct SeenNodes(HashSet<(Pos, Direction, usize)>);
impl Default for SeenNodes {
  fn default() -> Self {
    SeenNodes(HashSet::new())
  }
}
impl SeenNodes {
  pub fn see(&mut self, node: &Node) {
    self.0.insert((node.pos.clone(), node.direction, node.overheat));
  }

  pub fn check(&self, node: &Node) -> bool {
    self.0.get(&(node.pos.clone(), node.direction, node.overheat)).is_some()
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }
}
