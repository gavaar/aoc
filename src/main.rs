use std::{io::{self, Write},process};

mod advent;
mod shared;

fn main() {   
    let mut day_input = String::new();
    println!("\n\nWrite 'exit' to break program");

    loop {
        print!("{}", shared::Color::Blue("--> Day to test: "));
        io::stdout().flush().expect("??");
        io::stdin().read_line(&mut day_input).expect("??");
        let trimmed_day = day_input.trim();

        if trimmed_day == "exit" {
            println!("bye!");
            process::exit(1);
        }

        advent::run_day(day_input.trim());
        println!("----------------\n");
        day_input = "".to_string();
    }
}
