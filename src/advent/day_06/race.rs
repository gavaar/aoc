fn quadratic_equation(a: f64, b: f64, c: f64) -> (f64, f64) {
  let inner = b * b - 4f64 * a * c;
  let lower = 2f64 * a;

  (
    (-1f64 * b + inner.sqrt()) / lower,
    (-1f64 * b - inner.sqrt()) / lower,
  )
}

pub struct Race {
  pub record: f64,
  pub total_time: f64,
}
impl Race {
  /*
    time_pressed = x;
    total_time = b
    distance = c;

               [        time left        ]
    distance = (total_time - time_pressed) time_pressed; // time_pressed === speed;
    c = (b - x) x;

    0 = -x^2 + bx - c.
  */
  pub fn breakpoints_to_break_record(&self) -> (f64, f64) {
    quadratic_equation(-1f64, self.total_time, -1f64 * self.record)
  }
}
