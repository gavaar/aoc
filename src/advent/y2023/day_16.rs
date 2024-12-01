use crate::{shared::{read_input, print_test, print_solution, Color, report_progress}, advent::y2023::day_16::pos::Pos};

mod pos;
mod beam;
use beam::Beam;
mod contraption;
use contraption::Contraption;

fn fire_beams(mut contraption: &mut Contraption, first_beam: Beam) {
  let mut active_beams = vec![first_beam];

  loop {
    let mut new_beams: Vec<Beam> = Vec::new();

    for beam in active_beams.iter_mut() {
      let it_duplicates = beam.react_to_env(&contraption);
      if it_duplicates {
        new_beams.push(Beam::duplicate(beam));
      }
      beam.try_move(&mut contraption);
    }

    active_beams.append(&mut new_beams);

    for index in (0..active_beams.len()).rev() {
      let beam = &active_beams[index];

      if beam.finished {
        active_beams.swap_remove(index);
      }
    }

    if active_beams.len() == 0 {
      break;
    }
  }
}

fn part_one(input: &String) {
  let mut contraption = Contraption::new(input);

  fire_beams(&mut contraption, Beam::default());

  println!("tiles: {}", Color::Green(contraption.energized()));
}

fn part_two(input: &String) {
  let mut contraption = Contraption::new(input);
  let (total_rows, total_cols) = contraption.size();
  // let mut known_history: Vec<(Pos, Pos)> = vec![];
  let total = (total_rows + total_cols) * 2;
  let mut curr = 0;
  let mut diff_energizings: Vec<usize> = vec![];

  for col in 0..total_cols {
    for direction in ["top", "bot"] {
      // contraption.history.append(&mut known_history);
      
      let mut beam = Beam::default();
      let init_row = if direction == "bot" { 0 } else { total_rows as isize - 1 };
      beam.position = Pos(init_row, col as isize);
      beam.velocity = Pos::directed(direction);

      fire_beams(&mut contraption, beam);

      // known_history.append(&mut contraption.history);
      diff_energizings.push(contraption.energized());
      contraption.history.clear();
      report_progress(curr, total);
      curr += 1;
    }
  }

  for row in 0..total_rows {
    for direction in ["left", "right"] {
      // contraption.history.append(&mut known_history);
      
      let mut beam = Beam::default();
      let init_col = if direction == "right" { 0 } else { total_cols as isize - 1 };
      beam.position = Pos(row as isize, init_col);
      beam.velocity = Pos::directed(direction);

      fire_beams(&mut contraption, beam);

      // known_history.append(&mut contraption.history);
      diff_energizings.push(contraption.energized());
      contraption.history.clear();
      report_progress(curr, total);
      curr += 1;
    }
  }

  println!("biggest solution is: {}", Color::Red(diff_energizings.iter().max().unwrap()));
}

pub fn run() {
  print_test();
  let contraption_test = read_input("day_16/test");
  part_one(&contraption_test);
  part_two(&contraption_test);

  println!();

  print_solution();
  let contraption = read_input("day_16/input");
  part_one(&contraption);
  part_two(&contraption);
}
