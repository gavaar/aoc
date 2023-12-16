use crate::{shared::{read_input, print_test, print_solution, Color}, advent::day_16::pos::Pos};

mod pos;
mod beam;
use beam::Beam;
mod contraption;
use contraption::Contraption;

fn part_one(input: &String) {
  let mut contraption = Contraption::new(input);
  let mut finished_beams: Vec<Beam> = Vec::new();
  let mut active_beams = vec![Beam::default()];

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
        let finished_beam = active_beams.swap_remove(index);
        finished_beams.push(finished_beam);
      }
    }

    if active_beams.len() == 0 {
      break;
    }
  }

  let mut painted_points: Vec<Pos> = Vec::new();
  contraption.history.iter().for_each(|point| {
    if !painted_points.contains(&point.1) {
      painted_points.push(point.1);
    }
  });

  println!("{}", Color::Green(contraption.paint(&painted_points)));
  // println!("{:?}", contraption.history);
  println!("tiles: {}", Color::Green(painted_points.into_iter().count()));
}

pub fn run() {
  print_test();
  let contraption_test = read_input("day_16/test");
  part_one(&contraption_test);

  println!();

  print_solution();
  let contraption = read_input("day_16/input");
  part_one(&contraption);
}
