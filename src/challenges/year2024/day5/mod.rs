use std::{fs, io, path::Path};
use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = setup().unwrap();
    let (rules_section, updates_section) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_section);
    let updates = parse_updates(updates_section);

    let mut sum_of_valid_middle_pages = 0;
    let mut invalid_updates = Vec::new();

    for update in &updates {
        if is_update_valid(&rules, update) {
            if let Some(middle) = find_middle_page(update) {
                sum_of_valid_middle_pages += middle;
            }
        } else {
            invalid_updates.push(update.clone());
        }
    }

    println!("Sum of valid middle pages: {}", sum_of_valid_middle_pages);

    let mut sum_of_fixed_middle_pages = 0;

    for invalid_update in invalid_updates {
        let fixed_update = reorder_update(&rules, &invalid_update);
        if let Some(middle) = find_middle_page(&fixed_update) {
            sum_of_fixed_middle_pages += middle;
        }
    }

    println!("Sum of fixed middle pages: {}", sum_of_fixed_middle_pages);
}

fn setup() -> io::Result<String> {
    let file_path = Path::new("./src/challenges/year2024/day5/input.txt");
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

fn parse_rules(rules_section: &str) -> HashMap<u32, HashSet<u32>> {
    let mut rules = HashMap::new();

    for line in rules_section.lines() {
        if let Some((x, y)) = line.split_once('|') {
            let x: u32 = x.parse().unwrap();
            let y: u32 = y.parse().unwrap();

            rules.entry(x).or_insert_with(HashSet::new).insert(y);
        }
    }

    rules
}

fn parse_updates(updates_section: &str) -> Vec<Vec<u32>> {
    updates_section
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_update_valid(rules: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> bool {
    let positions: HashMap<u32, usize> = update.iter().enumerate().map(|(i, &page)| (page, i)).collect();

    for (&x, ys) in rules {
        for &y in ys {
            if let (Some(&pos_x), Some(&pos_y)) = (positions.get(&x), positions.get(&y)) {
                if pos_x > pos_y {
                    return false;
                }
            }
        }
    }

    true
}

fn find_middle_page(update: &[u32]) -> Option<u32> {
    if update.is_empty() {
        None
    } else {
        Some(update[update.len() / 2])
    }
}

fn reorder_update(rules: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> Vec<u32> {
    let mut dependencies = HashMap::new();

    for &page in update {
        dependencies.insert(page, HashSet::new());
    }

    for (&x, ys) in rules {
        if dependencies.contains_key(&x) {
            for &y in ys {
                if dependencies.contains_key(&y) {
                    dependencies.get_mut(&y).unwrap().insert(x);
                }
            }
        }
    }

    let mut ordered_pages = Vec::new();
    let mut no_dependencies: Vec<u32> = dependencies.iter()
        .filter(|(_, deps)| deps.is_empty())
        .map(|(&page, _)| page)
        .collect();

    while let Some(page) = no_dependencies.pop() {
        ordered_pages.push(page);

        for (&dependent_page, deps) in dependencies.iter_mut() {
            deps.remove(&page);
            if deps.is_empty() && !ordered_pages.contains(&dependent_page) {
                no_dependencies.push(dependent_page);
            }
        }
    }

    ordered_pages
}
