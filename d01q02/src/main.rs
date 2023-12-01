use std::fs;
use std::collections::HashMap;

const DEFAULT_FILE_PATH: &str = "./input.txt";
const RADIX: u32 = 10;

fn main() {
    let contents = fs::read_to_string(DEFAULT_FILE_PATH)
        .expect("Should have been able to read the file");

    // Map for converting words to numbers
    let word_to_num: HashMap<&str, i32> = HashMap::from([
        ("one", 1), ("two", 2), ("three", 3),
        ("four", 4), ("five", 5), ("six", 6),
        ("seven", 7), ("eight", 8), ("nine", 9)
    ]);
    let sum: i32 = contents.lines().map(|line| {
        let mut num1: i32 = -1;
        let mut num2: i32 = -1;

        for (i, c) in line.chars().enumerate() {
            let index = i;
            let mut  num: i32 = match c.to_digit(RADIX) {
                Some(n) => n as i32,
                None => -1,
            };

            // if char is a letter
            if num == -1 {
                if line[index..].len() < 3 {
                    continue;
                }
                if index + 3 <= line.len() && word_to_num.contains_key(&line[index..index+3]) {
                    num = *word_to_num.get(&line[index..index+3]).unwrap();
                } else if index + 4 <= line.len() && word_to_num.contains_key(&line[index..index+4]) {
                    num = *word_to_num.get(&line[index..index+4]).unwrap();
                } else if index + 5 <= line.len() && word_to_num.contains_key(&line[index..index+5]) {
                    num = *word_to_num.get(&line[index..index+5]).unwrap();
                } else {
                    continue;
                }
            }

            if num1 == -1 {
                num1 = num;
            } else {
                num2 = num;
            }
        }

        if num2 == -1 {
            num2 = num1;
        }

        let sum = num1.to_string() + &num2.to_string();
        sum.parse::<i32>().unwrap()
    }).sum();
    
    println!("Sum: {}", sum);
}
