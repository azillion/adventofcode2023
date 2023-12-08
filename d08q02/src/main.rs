use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Route {
    key: String,
    left_key: String,
    right_key: String,
}

impl Route {
    fn new(key: String, left_key: String, right_key: String) -> Route {
        Route {
            key,
            left_key,
            right_key,
        }
    }
}

fn main() {
    let start_time = std::time::Instant::now();
    let is_test = false;
    let contents = get_content(is_test);

    let mut directions = Vec::new();
    let mut routes_map = HashMap::new();
    let mut a_route_keys = Vec::new();

    contents.lines().enumerate().for_each(|(i, line)| {
        if i == 0 {
            for c in line.chars() {
                match c {
                    'L' => directions.push(Direction::Left),
                    'R' => directions.push(Direction::Right),
                    _ => (),
                }
            }
            return;
        }
        if line == "" {
            return;
        }
        let route_data = line
            .split(" = ")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        let route_key = route_data[0].clone();
        let routes = route_data[1]
            .split(", ")
            .map(|s| {
                s.trim()
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .to_string()
            })
            .collect::<Vec<String>>();

        let route = Route::new(route_key.clone(), routes[0].clone(), routes[1].clone());
        if route.key.ends_with("A") {
            a_route_keys.push(route_key.clone());
        }
        routes_map.insert(route_key, (0, route));
    });

    let mut trip_length = 0;
    loop {
        let route_keys = a_route_keys.clone();
        let direction = &directions[trip_length % directions.len()];
        let next_keys = route_keys
            .iter()
            .filter_map(|s| {
                let route = routes_map.get_mut(s).unwrap();
                if route.0 > 0 {
                    return None;
                }
                if route.1.key.ends_with("Z") {
                    route.0 = trip_length;
                    return None;
                }

                let next_key = match direction {
                    Direction::Left => &route.1.left_key,
                    Direction::Right => &route.1.right_key,
                };
                Some(next_key.clone())
            })
            .collect::<Vec<String>>();
        trip_length += 1;
        if next_keys.len() == 0 {
            break;
        }

        a_route_keys = next_keys;
    }

    let total_trip_length = routes_map
        .iter()
        .filter(|(_, (trip_length, _))| *trip_length > 0)
        .map(|trip_length| {
            println!(
                "trip length: {}, route: {}",
                trip_length.1 .0, trip_length.1 .1.key
            );
            trip_length.1 .0
        })
        .reduce(|a, b| lcm(a, b))
        .unwrap();

    println!("Trip length: {}", total_trip_length);
    println!("Run time: {:?}", start_time.elapsed());
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn get_content(is_test: bool) -> String {
    if is_test {
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            .to_string()
    } else {
        let filename = "input.txt";
        fs::read_to_string(filename).expect("Something went wrong reading the file")
    }
}
