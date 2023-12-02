use std::fs;
// use std::collections::HashMap;

const DEFAULT_FILE_PATH: &str = "./input.txt";
const RADIX: u32 = 10;
const RED_MAX: i32 = 12;
const GREEN_MAX: i32 = 13;
const BLUE_MAX: i32 = 14;

fn main() {
    let contents = fs::read_to_string(DEFAULT_FILE_PATH)
        .expect("Should have been able to read the file");

    let sum: i32 = contents.lines().map(|line| {
        let turn_lines = line.split(": ").collect::<Vec<&str>>();
        let game = turn_lines[0][5..].parse::<i32>().unwrap();
        let turns = turn_lines[1].split(";").map(|turn| turn.trim()).collect::<Vec<&str>>();

        let mut possible = true;
        turns.iter().for_each(|turn| {
            let (red, blue, green) = parse_cubes(turn);
            if red > RED_MAX || blue > BLUE_MAX || green > GREEN_MAX {
                possible = false;
            }
        });

        println!("Game: {}, Possible: {}", game, possible);

        if possible {
            return game;
        }

        0
    }).sum();

    println!("Sum: {}", sum);
}

fn parse_cubes(turn: &str) -> (i32, i32, i32) {
    let mut red_count = 0;
    let mut blue_count = 0;
    let mut green_count = 0;

    let cubes = turn.split(",").map(|cube| cube.trim()).collect::<Vec<&str>>();
    let cube_parts = cubes.iter().map(|cube| {
        let cube_parts = cube.split(" ").map(|part| part.trim()).collect::<Vec<&str>>();
        let cube_value = cube_parts[0].parse::<i32>().unwrap();
        (cube_value, cube_parts[1])
    }).collect::<Vec<(i32, &str)>>();
    cube_parts.iter().for_each(|(cube_value, cube_color)| {
        match cube_color {
            &"red" => red_count += cube_value,
            &"blue" => blue_count += cube_value,
            &"green" => green_count += cube_value,
            _ => panic!("Unknown cube color: {}", cube_color)
        }
    });

    (red_count, blue_count, green_count)
}
