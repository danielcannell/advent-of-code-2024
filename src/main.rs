use std::env;
use std::process::exit;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: incorrect number of arguments");
        eprintln!();
        eprintln!("Usage:");
        eprintln!("    advent-of-code-2024 <day>");
        exit(1);
    }

    if args[1] == "--help" || args[1] == "-h" {
        println!("Usage:");
        println!("    advent-of-code-2024 <day>");
        return;
    }

    let day: u32 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid day");
        exit(1);
    });

    match day {
        1 => day1::solve(),
        2 => day2::solve(),

        _ => {
            eprintln!("I haven't solved that day yet!");
            exit(1);
        }
    };
}
