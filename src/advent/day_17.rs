use crate::shared::{print_test, read_input, Color, print_solution, ToU8Map, report_progress};

mod direction;
mod pos;
use pos::Pos;
mod node;
use node::Node;
mod seen_nodes;
mod better_queue;
use better_queue::BetterQueue;
mod helpers;
use helpers::draw;

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
