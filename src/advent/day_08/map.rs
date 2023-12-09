use std::collections::HashMap;

#[derive(Debug)]
struct Node<'a> {
  id: &'a str,
  left: &'a str,
  right: &'a str,
}
impl<'a> Node<'a> {
  pub fn new(line: &'a str) -> Node<'a> {
    let (id, directions) = line.split_once(" = ").unwrap();
    let (left, right) = directions[1..directions.len() - 1].split_once(", ").unwrap();

    Node {
      id,
      left,
      right,
    }
  }

  pub fn next(&self, step: char) -> &'a str {
    match step {
      'L' => self.left,
      'R' => self.right,
      _ => "unreachable",
    }
  }
}

#[derive(Debug)]
pub struct Map<'a> {
  steps: Vec<char>,
  nodes: HashMap<&'a str, Node<'a>>,
  pub current_node: &'a str,
  pub steps_taken: usize,
}
impl<'a> Map<'a> {
  pub fn new(paper: &String) -> Map {
    let mut lines = paper.lines();

    let steps = lines.next().unwrap().chars().collect();

    lines.next(); // clear one line

    let mut nodes = HashMap::new();

    for line in lines {
      nodes.insert(line.split_at(3).0, Node::new(line));
    }

    Map {
      steps,
      nodes,
      current_node: "AAA",
      steps_taken: 0usize,
    }
  }

  pub fn next(&mut self) -> &'a str {
    let node = self.nodes.get(self.current_node).unwrap();
    let step = self.steps[self.steps_taken % self.steps.len()];
    let next_step = node.next(step);

    self.steps_taken += 1;
    self.current_node = next_step;

    next_step
  }
}
