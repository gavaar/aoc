use std::collections::HashMap;

pub struct PageUpdater {
  rules: HashMap<(u32, u32), bool>,
  updates: Vec<Vec<u32>>,
}
impl PageUpdater {
  pub fn new(str_input: &String) -> PageUpdater {
    let (rules_str, updates_str) = str_input.split_once("\n\n").expect("input is wrong wtf");

    let updates: Vec<Vec<u32>> = updates_str.lines()
      .map(|line| {
        line.split(",")
          .map(|num_str| num_str.parse().expect("num is not num?"))
          .collect()
      })
      .collect();

    let mut rules: HashMap<(u32, u32), bool> = HashMap::new();
    rules_str.lines().for_each(|line| {
      let (before_str, after_str) = line.split_once("|").expect("I cant think properly");
      let before: u32 = before_str.parse().expect("before not a num");
      let after: u32 = after_str.parse().expect("after not a num");
      rules.insert((before, after), true);
      rules.insert((after, before), false);
    });

    PageUpdater { rules, updates }
  }

  pub fn mid_page(vec: &Vec<u32>) -> u32 {
    let idx = vec.len() - 1;
    vec[idx / 2]
  }

  pub fn correct_middle_pages(&self) -> Vec<u32> {
    let mut correct_middle_pages = Vec::new();
    self.updates.iter().for_each(|update_vec| {
      let mut no_errors = true;

      for update in 0..update_vec.len() {
        for page in (1 + update)..update_vec.len() {
          let x = update_vec[update];
          let y = update_vec[page];

          let valid_rule = *self.rules.get(&(x, y))
                                      .expect(format!("rule ({page},{update}) does not exist").as_str());
          if !valid_rule {
            no_errors = false;
          }
        }
      }

      if no_errors {
        let middle_page = PageUpdater::mid_page(&update_vec);
        correct_middle_pages.push(middle_page);
      }
    });

    correct_middle_pages
  }
}
