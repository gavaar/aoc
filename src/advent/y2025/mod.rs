mod day_01;
mod day_02;
mod day_03;
mod day_04;

pub fn run_day(buffer: &str) {        
  if let Ok(day) = buffer.parse::<u8>() {

  match day {
    1 => day_01::run(),
    2 => day_02::run(),
    3 => day_03::run(),
    4 => day_04::run(),
    _ => println!("We did not find that day\n"),
   }
  } else {
    println!("{:?} is not a number\n", buffer);
  }
}
