use std::collections::BTreeSet;
use std::fs;

const DEFAULT_FILE_PATH: &str = "./input.txt";

#[derive(Debug, Clone, Copy)]
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

    fn source_to_destination(&self, source: i64) -> Option<i64> {
        if self.source <= source && source < (self.source + self.range) {
            Some(source - self.source + self.destination)
        } else {
            None
        }
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
    maps: Vec<Map>,
}

impl Maps {
    fn new() -> Self {
        Self { maps: Vec::new() }
    }

    fn add_map(&mut self, map: Map) {
        self.maps.push(map);
    }

    fn convert(&self, source: i64) -> i64 {
        match self
            .maps
            .iter()
            .map(|entry| entry.source_to_destination(source))
            .find_map(|e| e)
        {
            Some(dest) => dest,
            None => source,
        }
    }

    fn convert_range(&self, range: (i64, i64)) -> Vec<(i64, i64)> {
        let mut slices = BTreeSet::new();
        let range_end = range.0 + range.1;

        for entry in &self.maps {
            let source_end = entry.source + entry.range;

            if range_end < entry.source || range.0 > source_end {
                continue;
            }

            if entry.source > range.0 {
                slices.insert(entry.source);
            }

            if source_end < range_end {
                slices.insert(source_end);
            }
        }
        slices.insert(range_end);

        let mut output = Vec::new();
        let mut current = range.0;

        for position in slices {
            output.push((self.convert(current), position - current));
            current = position;
        }

        output
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

    let all_maps = vec![
        &seed_to_soil,
        &soil_to_fertilizer,
        &fertilizer_to_water,
        &water_to_light,
        &light_to_temperature,
        &temperature_to_humidity,
        &humidity_to_location,
    ];

    let mut current = seeds
        .iter()
        .map(|x| (x.seed, x.range))
        .collect::<Vec<(i64, i64)>>();
    let mut future = Vec::new();

    for maps in all_maps {
        for seed in &current {
            future.extend(maps.convert_range(*seed));
        }
        current = future;
        future = Vec::new();
    }

    let min_location = current.iter().map(|x| x.0).min().unwrap_or(-1);

    println!("Lowest Seed Location: {}", min_location);
    let full_elapsed = full_start.elapsed();
    println!("Total Run Time: {:?}", full_elapsed);
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

#[allow(dead_code)]
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
