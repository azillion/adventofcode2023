use std::collections::{HashMap, HashSet};
use std::fs;

const DEFAULT_FILE_PATH: &str = "./input.txt";

fn main() {
    let contents =
        fs::read_to_string(DEFAULT_FILE_PATH).expect("Should have been able to read the file");

    let mut hm = HashMap::new();

    contents.lines().for_each(|line| {
        let card = line
            .split(": ")
            .map(|part| part.trim())
            .collect::<Vec<&str>>();

        let mut card_id = 0;
        card[0].split(" ").for_each(|s| {
            let trimmed_s = s.trim();
            if trimmed_s.is_empty() {
                return;
            }
            if trimmed_s == "Card" {
                return;
            }
            let num = trimmed_s
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Could not parse: {}", trimmed_s));
            card_id = num;
        });
        if hm.contains_key(&card_id) {
            hm.insert(card_id, hm.get(&card_id).unwrap() + 1);
        } else {
            hm.insert(card_id, 1);
        }
        let card_parts = card[1]
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

        let mut winning_count = 0;
        card_parts[1].trim().split(" ").for_each(|s| {
            let trimmed_s = s.trim();
            if trimmed_s.is_empty() {
                return;
            }
            let num = trimmed_s
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Could not parse: {}", trimmed_s));
            if winning_numbers_map.contains(&num) {
                winning_count += 1;
            }
        });
        if winning_count == 0 {
            return;
        }
        for k in 1..*hm.get(&card_id).unwrap() + 1 {
            for i in 1..winning_count + 1 {
                let card_id = card_id + i;
                if hm.contains_key(&card_id) {
                    hm.insert(card_id, hm.get(&card_id).unwrap() + 1);
                } else {
                    hm.insert(card_id, 1);
                }
            }
        }
    });

    let mut total_scratch_cards = 0;
    hm.iter().for_each(|(k, v)| {
        total_scratch_cards += v;
    });

    println!("Sum: {}", total_scratch_cards);
}
