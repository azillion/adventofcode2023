use std::collections::hash_map;
use std::fs;

const START_ROUTE: &str = "AAA";
const END_ROUTE: &str = "ZZZ";

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
    let mut routes_map = hash_map::HashMap::new();
    let mut first_route = None;
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
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        let route_key = route_data[0].clone();
        let routes = route_data[1]
            .split(", ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| {
                s.trim()
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .to_string()
            })
            .collect::<Vec<String>>();
        let origin_route = Route::new(route_key.clone(), routes[0].clone(), routes[1].clone());
        if first_route.is_none() && origin_route.key == START_ROUTE {
            first_route = Some(origin_route.clone());
        }
        routes_map.insert(route_key, origin_route);
    });

    let trip_length = find_route(&first_route.unwrap(), &directions, &routes_map, 0);
    println!("Trip length: {}", trip_length);
    println!("Run time: {:?}", start_time.elapsed());
}

fn find_route(
    current_route: &Route,
    directions: &[Direction],
    routes_map: &hash_map::HashMap<String, Route>,
    trip_length: usize,
) -> usize {
    if current_route.key == END_ROUTE {
        return trip_length;
    }

    if let Some(direction) = directions.get(trip_length % directions.len()) {
        let next_route = match direction {
            Direction::Left => routes_map.get(&current_route.left_key),
            Direction::Right => routes_map.get(&current_route.right_key),
        };

        match next_route {
            Some(route) => find_route(&route, &directions, routes_map, trip_length + 1),
            None => trip_length,
        }
    } else {
        trip_length
    }
}

fn get_content(is_test: bool) -> String {
    if is_test {
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            .to_string()
    } else {
        let filename = "input.txt";
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        return contents;
    }
}
