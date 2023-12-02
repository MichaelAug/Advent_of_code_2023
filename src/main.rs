use std::{env, fs};

mod day;
mod day_1;

use day::Day;
use day_1::Day1;

use crate::day::DayError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} -- <day>", args[0]);
        return Err("Invalid number of arguments".into());
    }

    println!("Running day {}", args[1]);

    let day: usize = args[1].parse().expect("Invalid day number");

    // Read input file for the specified day
    let input_filename = format!("inputs/day{:02}.txt", day);
    let input_content = fs::read_to_string(&input_filename)
        .expect(&format!("Failed to read input file for day {}", day));

    let solution = match day {
        1 => Day1.solve(&input_content),
        _ => Err(DayError::Input(format!("Incorrect day specified: {day}"))),
    }?;

    println!("{solution}");
    Ok(())
}
