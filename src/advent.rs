mod day_01;

pub fn run_day(buffer: &str) {        
  if let Ok(day) = buffer.parse::<u8>() {
   match day {
    1 => day_01::run(),
    _ => println!("We did not find that day\n"),
   } 
  } else {
    println!("{:?} is not a number\n", buffer);
  }
}
