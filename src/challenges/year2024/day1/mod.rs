use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let (list1, list2) = match setup() {
        Ok(lists) => lists,
        Err(e) => {
            eprintln!("Error during setup: {}", e);
            return;
        }
    };

    let total_distance = calculate_total_distance(list1.clone(), list2.clone());
    println!("Total distance is {:?}", total_distance);

    let similarity_score = calculate_similarity_score(list1, list2);

    println!("Similarity score {:?}", similarity_score);
}

fn setup() -> io::Result<(Vec<i64>, Vec<i64>)> {
    let file_path = Path::new("./src/challenges/year2024/day1/input.txt");
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    
    for line in reader.lines() {
        let ln = line?;
        let parts: Vec<&str> = ln.split("   ").collect();

        let num1 = parts[0]
            .parse::<i64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let num2 = parts[1]
            .parse::<i64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        list1.push(num1);
        list2.push(num2);
    }

    Ok((list1, list2))
}

fn calculate_total_distance(list1: Vec<i64>, list2: Vec<i64>) -> i64 {
    let mut sorted_list1 = list1;
    let mut sorted_list2 = list2;

    sorted_list1.sort();
    sorted_list2.sort();

    let mut total_distance: i64 = 0;
    for (location1, location2) in sorted_list1.iter().zip(sorted_list2.iter()) {
        total_distance += (location1 - location2).abs();
    }
    total_distance
}

fn calculate_similarity_score(list1: Vec<i64>, list2: Vec<i64>) -> i64 {
    let mut similarity_score = 0;
    
    for &location in &list1 {
        let count = list2.iter().filter(|&&x| x == location).count() as i64;
        similarity_score += count * location;
    }

    similarity_score
}
