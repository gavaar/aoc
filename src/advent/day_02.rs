mod game;
pub use game::{Game, Round};

use crate::shared::{read_input, print_test, print_solution, Color};

fn parse_input(file: &str, bag: Round) -> Vec<Game>  {
  let read_file = read_input(file);
  let mut games: Vec<Game> = Vec::new();

  for line in read_file.lines() {
    games.push(Game::new(line, &bag));
  }

  games
}

fn add_game_ids(file: &str) -> usize {
  let games = parse_input(file, Round { blue: 14, green: 13, red: 12 });
  let mut count = 0;
  for game in games {
    if game.is_possible {
      count += game.id;
    }
  }
  count
}

fn add_game_powers(file: &str) -> usize {
  let games = parse_input(file, Round { blue: 0, green: 0, red: 0 });
  let mut count = 0;
  for game in games {
    count += game.power;
  }
  count
}

pub fn run() {
  print_test();
  println!("The value is: {}", Color::Red(add_game_ids("day_02/test")));
  println!("The power sum is: {}\n", Color::Red(add_game_powers("day_02/test")));
  
  print_solution();
  println!("The value of game 1 is: {}", Color::Red(add_game_ids("day_02/input")));
  println!("The power sum is: {}\n", Color::Red(add_game_powers("day_02/input")));
}
