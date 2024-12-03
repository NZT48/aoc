mod challenges;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "2024-day1" => challenges::year2024::day1::run(),
            "2024-day2" => challenges::year2024::day2::run(),
            "2024-day3" => challenges::year2024::day3::run(),
            _ => println!("Unknown day: {}", args[1]),
        }
    } else {
        println!("Usage: cargo run <year-day>");
    }
}