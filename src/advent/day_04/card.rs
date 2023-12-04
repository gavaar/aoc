use std::collections::HashSet;

#[derive(Debug)]
pub struct Card {
  // id: usize,
  winning_numbers: HashSet<String>,
  held_numbers: HashSet<String>,
}
impl Card {
  pub fn new(card_row: &str) -> Card {
    let mut split = card_row.split(":");
    let _id = split.next() // we err handle here, altho we don't expect this to fail.
                  .unwrap_or("C 0")
                  .split(" ")
                  .last()
                  .unwrap_or("0")
                  .parse::<usize>()
                  .expect("id was not a number");

    let mut numbers = split.next()
                       .expect("something went wrong when getting numbers")
                       .split("|");
    
    let winning_numbers = numbers.next()
                                 .expect("winning numbers error")
                                 .trim()
                                 .split(" ")
                                 .into_iter()
                                 .filter_map(|n| if n.len() > 0 { Some(n.to_string()) } else { None } )
                                 .collect();

    let held_numbers = numbers.next()
                              .expect("held numbers error")
                              .trim()
                              .split(" ")
                              .into_iter()
                              .filter_map(|n| if n.len() > 0 { Some(n.to_string()) } else { None } )
                              .collect();

    Card {
      // id,
      winning_numbers,
      held_numbers,
    }
  }

  pub fn winning_score(&self) -> u32 {
    let mut score = 0;

    for number in &self.held_numbers {
      if self.winning_numbers.contains(number) {
        if &score == &0 {
          score = 1;
        } else {
          score = score << 1;
        }
      }
    }

    score
  }
}
