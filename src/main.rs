mod days;

use clap::Parser;

use days::{day1, day2, day3, day4, day5, day6};

#[derive(Parser)]
#[command(
    author = "Michal K. <michal0kasprzyk@gmail.com>",
    version = "0.1",
    about = "Rust Solutions Advent of Code 2022"
)]
struct Cli {
    days: Vec<i32>,
}

fn main() {
    let cli = Cli::parse();
    for day in cli.days {
        let day_runner = match day {
            1 => day1::run,
            2 => day2::run,
            3 => day3::run,
            4 => day4::run,
            5 => day5::run,
            6 => day6::run,
            _ => {
                eprintln!("Incorrect day value `{}`. Skipping", day);
                continue;
            }
        };
        println!("[Day {}] Running..", day);
        day_runner();
    }
}
