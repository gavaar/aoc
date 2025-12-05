use std::collections::HashMap;

struct Ingredient {
  pub available: bool,
  pub fresh: bool,
}
impl Default for Ingredient {
  fn default() -> Self {
    Ingredient { available: false, fresh: false }
  }
}

pub struct Inventory {
  ingredients: HashMap<usize, Ingredient>,
}
impl Default for Inventory {
  fn default() -> Self {
    Inventory { ingredients: HashMap::new() }
  }
}
impl Inventory {
  pub fn mark_as_available(&mut self, ingredient_id: &usize) {
    let ingredient = self.get_or_create(ingredient_id);
    ingredient.available = true;
  }

  pub fn mark_as_fresh(&mut self, ingredient_id: &usize) {
    let ingredient = self.get_or_create(ingredient_id);
    ingredient.fresh = true;
  }

  pub fn get_all_ingredients(&self) -> Vec<usize> {
    self.ingredients.keys().map(|k| *k).collect()
  }

  pub fn fresh_available_ingredients(&self) -> Vec<usize> {
    self.ingredients.iter().filter_map(|ing| {
      let (id, ingredient) = ing;
      if ingredient.available && ingredient.fresh {
        return Some(*id);
      }

      return None;
    }).collect()
  }

  fn get_or_create(&mut self, id: &usize) -> &mut Ingredient {
    let has_ingr = self.ingredients.contains_key(&id);
    if !has_ingr {
      self.ingredients.insert(*id, Ingredient::default());
    }
    
    return self.ingredients.get_mut(id).expect("I just created it, should exist");
  }
}
