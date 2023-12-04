use std::collections::HashSet;
use std::fs;

const DEFAULT_FILE_PATH: &str = "./input.txt";

fn main() {
    let contents =
        fs::read_to_string(DEFAULT_FILE_PATH).expect("Should have been able to read the file");

    let sum: i32 = contents
        .lines()
        .map(|line| {
            let mut sum = 0;
            let card = line
                .split(": ")
                .map(|part| part.trim())
                .collect::<Vec<&str>>()[1];
            let card_parts = card
                .split(" | ")
                .map(|part| part.trim())
                .collect::<Vec<&str>>();
            let mut winning_numbers_map = HashSet::new();
            card_parts[0].trim().split(" ").for_each(|s| {
                let trimmed_s = s.trim();
                if trimmed_s.is_empty() {
                    return;
                }
                let num = trimmed_s
                    .parse::<i32>()
                    .unwrap_or_else(|_| panic!("Could not parse: {}", trimmed_s));
                winning_numbers_map.insert(num);
            });
            card_parts[1].trim().split(" ").for_each(|s| {
                let trimmed_s = s.trim();
                if trimmed_s.is_empty() {
                    return;
                }
                let num = trimmed_s
                    .parse::<i32>()
                    .unwrap_or_else(|_| panic!("Could not parse: {}", trimmed_s));
                if winning_numbers_map.contains(&num) {
                    if sum == 0 {
                        sum = 1;
                    } else {
                        sum *= 2;
                    }
                }
            });
            sum
        })
        .sum();

    println!("Sum: {}", sum);
}
