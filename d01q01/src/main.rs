use std::fs;

const DEFAULT_FILE_PATH: &str = "./input.txt";
const RADIX: u32 = 10;

fn main() {
    let contents = fs::read_to_string(DEFAULT_FILE_PATH)
        .expect("Should have been able to read the file");
    let sum: i32 = contents.lines().map(|line| {
        let mut num1: i32 = -1;
        let mut num2: i32 = -1;

        for c in line.chars() {
            let num: i32 = match c.to_digit(RADIX) {
                Some(n) => n as i32,
                None => continue,
            };
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
