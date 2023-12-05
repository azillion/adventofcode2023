use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

const DEFAULT_FILE_PATH: &str = "./input.txt";

struct SeedData {
    seed: i64,
    range: i64,
}

impl SeedData {
    fn new(seed: i64, range: i64) -> Self {
        Self { seed, range }
    }
}

#[derive(Debug, Clone, Copy)]
struct Map {
    source: i64,
    destination: i64,
    range: i64,
}

impl Map {
    fn new(source: i64, destination: i64, range: i64) -> Self {
        Self {
            source,
            destination,
            range,
        }
    }

    fn invert(&self) -> HashMap<i64, i64> {
        (0..self.range)
            .into_par_iter()
            .map(|i| {
                let src = self.source + i;
                let dest = self.destination + i;
                (dest, src)
            })
            .collect()
    }
}

enum MapType {
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

struct Maps {
    maps: Vec<HashMap<i64, i64>>,
}

impl Maps {
    fn new() -> Self {
        Self { maps: Vec::new() }
    }

    fn add_map(&mut self, map: Map) {
        println!("Adding map: {:?}", map);
        self.maps.push(map.invert()); // Store inverted maps
    }

    // Iterative binary search
    fn map(&self, num: i64) -> i64 {
        let mut result = num;
        for map in &self.maps {
            result = *map.get(&result).unwrap_or(&result); // Use binary search
        }
        result
    }
}

fn main() {
    let full_start = std::time::Instant::now();
    let contents =
        fs::read_to_string(DEFAULT_FILE_PATH).expect("Should have been able to read the file");

    // let seeds = parse_seeds(test_data());
    let seeds = parse_seeds(&contents);

    let mut seed_to_soil = Maps::new();
    let mut soil_to_fertilizer = Maps::new();
    let mut fertilizer_to_water = Maps::new();
    let mut water_to_light = Maps::new();
    let mut light_to_temperature = Maps::new();
    let mut temperature_to_humidity = Maps::new();
    let mut humidity_to_location = Maps::new();

    let mut current_type = MapType::Soil;
    contents.lines().enumerate().for_each(|line| {
        if line.0 == 0 {
            return;
        }
        if line.1 == "" || line.1.starts_with("\n") {
            return;
        }
        if line.1.starts_with("seed-to-soil map:") {
            current_type = MapType::Soil;
            return;
        }
        if line.1.starts_with("soil-to-fertilizer map:") {
            current_type = MapType::Fertilizer;
            return;
        }
        if line.1.starts_with("fertilizer-to-water map:") {
            current_type = MapType::Water;
            return;
        }
        if line.1.starts_with("water-to-light map:") {
            current_type = MapType::Light;
            return;
        }
        if line.1.starts_with("light-to-temperature map:") {
            current_type = MapType::Temperature;
            return;
        }
        if line.1.starts_with("temperature-to-humidity map:") {
            current_type = MapType::Humidity;
            return;
        }
        if line.1.starts_with("humidity-to-location map:") {
            current_type = MapType::Location;
            return;
        }
        let values = line
            .1
            .split_whitespace()
            .map(|x| x.parse::<i64>().expect("Unable to parse number"))
            .collect::<Vec<i64>>();
        if values.len() == 3 {
            let map = Map::new(values[1], values[0], values[2]);
            match current_type {
                MapType::Soil => seed_to_soil.add_map(map),
                MapType::Fertilizer => soil_to_fertilizer.add_map(map),
                MapType::Water => fertilizer_to_water.add_map(map),
                MapType::Light => water_to_light.add_map(map),
                MapType::Temperature => light_to_temperature.add_map(map),
                MapType::Humidity => temperature_to_humidity.add_map(map),
                MapType::Location => humidity_to_location.add_map(map),
            }
        }
    });

    let min_location = Arc::new(AtomicI64::new(i64::MAX));
    seeds.par_iter().for_each(|seed| {
        let seed_start = seed.seed;
        let seed_end = seed.seed + seed.range;
        for i in seed_start..seed_end {
            let soil = seed_to_soil.map(i);
            let fertilizer = soil_to_fertilizer.map(soil);
            let water = fertilizer_to_water.map(fertilizer);
            let light = water_to_light.map(water);
            let temperature = light_to_temperature.map(light);
            let humidity = temperature_to_humidity.map(temperature);
            let location = humidity_to_location.map(humidity);

            min_location.fetch_min(location, Ordering::SeqCst);
        }
    });

    let full_elapsed = full_start.elapsed();
    println!("Total Run Time: {:?}", full_elapsed);
    println!(
        "Lowest Seed Location: {}",
        min_location.load(Ordering::SeqCst)
    );
}

fn parse_seeds(contents: &str) -> Vec<SeedData> {
    let mut result = Vec::new();
    let seed_line = contents.lines().next().unwrap_or("");
    let seeds = seed_line
        .split("seeds:")
        .nth(1)
        .unwrap_or("")
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for chunk in seeds.chunks(2) {
        if let [seed, range] = chunk {
            result.push(SeedData::new(*seed, *range));
        }
    }

    result
}

fn test_data() -> &'static str {
    return "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
}
