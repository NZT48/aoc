use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let reports = match setup() {
        Ok(lists) => lists,
        Err(e) => {
            eprintln!("Error during setup: {}", e);
            return;
        }
    };

    let mut safety_counter = 0;
    let mut dampener_counter = 0;
    for report in reports {
        if safety_check(report.clone()) {
            safety_counter +=1;
        } else {
            if dampener_safety_check(report) {
                dampener_counter += 1;
            }
        }
    }

    println!("Number of safe report is {:?}", safety_counter);
    println!("Number of safe reports including Dampener handling is {:?}", safety_counter + dampener_counter);
}

fn setup() -> io::Result<Vec<Vec<u32>>> {
    let file_path = Path::new("./src/challenges/year2024/day2/input.txt");
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();
    
    for line in reader.lines() {
        let ln = line?;

        let numbers: Vec<u32> = ln
            .split_whitespace() 
            .map(|num| {
                num.parse::<u32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .collect::<Result<_, _>>()?;

        result.push(numbers);
    }

    Ok(result)
}

fn safety_check(report: Vec<u32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut increasing = None;
    for level in report.windows(2) {
        let diff = (level[1] as i32 - level[0] as i32).abs();

        if diff < 1 || diff > 3 {
            return false;
        }

        if level[1] > level[0] {
            if increasing == Some(false) {
                return false;
            }
            increasing = Some(true);
        } else if level[1] < level[0] {
            if increasing == Some(true) {
                return false;
            }
            increasing = Some(false);
        }
    }

    true
}

fn dampener_safety_check(report: Vec<u32>) -> bool {
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);

        if safety_check(modified_report) {
            return true;
        }
    }

    false
}