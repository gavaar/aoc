use std::fs;

struct Round {
  pub blue: usize,
  pub red: usize,
  pub green: usize,
}

struct Game {
  pub id: usize,
  pub rounds: Vec<Round>,
  pub is_possible: bool,
  pub power: usize,
}
impl Game {
  fn new(line: &str, bag: &Round) -> Game {
    let mut split_line = line.split(':');
    let mut rounds = Vec::new();
    let mut is_possible = true;
    let mut min_value = Round {
      blue: 0,
      green: 0,
      red: 0,
    };

    let id = split_line.next().unwrap().split(' ').nth(1).unwrap().parse::<usize>().expect("can't usize the game id");
    let plays_iter = split_line.next().unwrap().split(';');

    for round_str in plays_iter {
      let mut round = Round {
        blue: 0,
        green: 0,
        red: 0,
      };

      let plays = round_str.split(',');
      for play in plays {
        let (num, color) = play.trim().split_once(' ').expect("something broke when splitting play");

        match color {
          "blue" => {
            round.blue = num.parse().unwrap();
          }
          "red" => {
            round.red = num.parse().unwrap();
          }
          "green" => {
            round.green = num.parse().unwrap();
          }
          _ => eprintln!("color did not match any branch"),
        }

        if round.blue > bag.blue || round.red > bag.red || round.green > bag.green {
          is_possible = false;
        }

        if round.blue > min_value.blue {
          min_value.blue = round.blue;
        }
        if round.green > min_value.green {
          min_value.green = round.green;
        }
        if round.red > min_value.red {
          min_value.red = round.red;
        }
      }

      rounds.push(round);
    }

    let power = min_value.red * min_value.blue * min_value.green;

    Game {
      id,
      rounds,
      is_possible,
      power,
    }
  }
}

fn parse_input(file: &str, bag: Round) -> Vec<Game>  {
  let read_file = fs::read_to_string(file).expect("error reading file");
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
  println!("### PART ONE ###");
  println!("-- Test --");
  println!("The value is: {}", add_game_ids("src/advent/day_02/test"));
  println!("The power sum is: {}", add_game_powers("src/advent/day_02/test"));
  
  println!("-- Solution part 1 --");
  println!("The value of game 1 is: {}", add_game_ids("src/advent/day_02/input"));
  println!("The power sum is: {}", add_game_powers("src/advent/day_02/input"));
}
