use std::{fs::File, io::{self, BufRead}, path::Path};


pub fn run() {
    let grid = setup();
    let word = "XMAS";

    let count = count_xmas(&grid, word);
    println!("The word '{}' appears {} times.", word, count);
}


fn setup() -> Vec<Vec<char>> {
    let file_path = "./src/challenges/year2024/day4/input.txt";

    let mut grid = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                grid.push(row.chars().collect());
            }
        }
    } else {
        panic!("Could not open file: {}", file_path);
    }

    grid
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_xmas(grid: &[Vec<char>], word: &str) -> usize {
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (0, -1),  // Left
        (-1, 0),  // Up
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
        (-1, 1),  // Diagonal up-right
        (-1, -1), // Diagonal up-left
    ];
    let word_chars: Vec<char> = word.chars().collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word_chars.len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                let mut found = true;
                for k in 0..word_len {
                    let nx = row as isize + k as isize * dx;
                    let ny = col as isize + k as isize * dy;
                    if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
                        found = false;
                        break;
                    }
                    if grid[nx as usize][ny as usize] != word_chars[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}
