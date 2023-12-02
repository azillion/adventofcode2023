use std::fs;
// use std::collections::HashMap;

const DEFAULT_FILE_PATH: &str = "./input.txt";

fn main() {
    let contents = fs::read_to_string(DEFAULT_FILE_PATH)
        .expect("Should have been able to read the file");

    let sum: i32 = contents.lines().map(|line| {
        let turn_lines = line.split(": ").collect::<Vec<&str>>();
        let turns = turn_lines[1].split(";").map(|turn| turn.trim()).collect::<Vec<&str>>();

        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;
        turns.iter().for_each(|turn| {
            let (red, blue, green) = parse_cubes(turn);
            min_red = min_red.max(red);
            min_blue = min_blue.max(blue);
            min_green = min_green.max(green);
        });
        min_red * min_blue * min_green
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
