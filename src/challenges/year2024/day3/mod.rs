use std::{fs, io, path::Path};

use regex::Regex;

pub fn run() {
    let memory = match setup() {
        Ok(lists) => lists,
        Err(e) => {
            eprintln!("Error during setup: {}", e);
            return;
        }
    };

    let multiplication_result = sum_multiplications(memory.clone());
    println!("Total sum of all products is {}", multiplication_result);


    let enabled_multiplications_result = calculate_enabled_multiplications(memory);
    println!("Total sum of all products from enabled chunks is {}", enabled_multiplications_result);
}

fn setup() -> io::Result<String> {
    let file_path = Path::new("./src/challenges/year2024/day3/input.txt");
    let content = fs::read_to_string(file_path)?;

    Ok(content)
}

fn sum_multiplications(memory: String) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total = 0;
    for cap in re.captures_iter(&memory) {
        let x: u32 = cap[1].parse().unwrap();
        let y: u32 = cap[2].parse().unwrap();
        let product = x * y;
        total += product;
    }

    return total;
}

fn find_enabled_chunks(input: &str) -> Vec<String> {
    let extended_input = format!("do(){}don't()", input);

    let mut substrings = Vec::new();
    let mut start_idx = 0;
    while let Some(start) = extended_input[start_idx..].find("do()") {
        // Find the position of "don't()" after "do()"
        if let Some(end) = extended_input[start_idx + start..].find("don't()") {
            let substring = &extended_input[start_idx + start..start_idx + start + end + "don't()".len()];
            substrings.push(substring.to_string());
            start_idx += start + end + "don't()".len();
        } else {
            break;
        }
    }

    substrings
}

fn calculate_enabled_multiplications(memory: String) -> u32 {
    let enabled_chunks = find_enabled_chunks(&memory);
    let mut total = 0;

    for memory_chunk in enabled_chunks {
        total += sum_multiplications(memory_chunk);
    }

    total
}