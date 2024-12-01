use std::{io::{self, Write},process, env};

mod advent;
mod shared;

fn main() {   
    let mut day_input = String::new();
    let mut args = env::args();

    if let Some(day_to_run) = args.nth(1) {
        advent::run_day(day_to_run.as_str());
        return;
    }

    loop {
        print!("{}", shared::Color::Blue("--> Day to test: "));
        io::stdout().flush().expect("??");
        io::stdin().read_line(&mut day_input).expect("??");
        let trimmed_day = day_input.trim();

        if trimmed_day == "exit" {
            println!("bye!");
            process::exit(1);
        }

        advent::run_day(trimmed_day);
        println!("----------------\n");
        day_input = "".to_string();
    }
}
