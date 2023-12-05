use std::fs;

const DEFAULT_FILE_PATH: &str = "./input.txt";

struct SeedData {
    seed: i64,
    soil: i64,
    fertilizer: i64,
    water: i64,
    light: i64,
    temperature: i64,
    humidity: i64,
    location: i64,
}

impl SeedData {
    fn new(seed: i64) -> Self {
        Self {
            seed,
            soil: -1,
            fertilizer: -1,
            water: -1,
            light: -1,
            temperature: -1,
            humidity: -1,
            location: -1,
        }
    }

    fn set_soil(&mut self, soil: i64) {
        self.soil = soil;
    }

    fn set_fertilizer(&mut self, fertilizer: i64) {
        self.fertilizer = fertilizer;
    }

    fn set_water(&mut self, water: i64) {
        self.water = water;
    }

    fn set_light(&mut self, light: i64) {
        self.light = light;
    }

    fn set_temperature(&mut self, temperature: i64) {
        self.temperature = temperature;
    }

    fn set_humidity(&mut self, humidity: i64) {
        self.humidity = humidity;
    }

    fn set_location(&mut self, location: i64) {
        self.location = location;
    }
}

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

    fn source_to_destination(&self, num: i64) -> i64 {
        if num >= self.source && num <= self.source + self.range {
            return self.destination + (num - self.source);
        }
        num
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

    fn map(&self, num: i64) -> i64 {
        let mut result = num;
        self.maps.iter().for_each(|map| {
            if num != result {
                return;
            }
            let destination = map.source_to_destination(result);
            if destination != result {
                result = destination;
            }
        });
        result
    }
}

fn main() {
    let contents =
        fs::read_to_string(DEFAULT_FILE_PATH).expect("Should have been able to read the file");

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
        let mut values: (i64, i64, i64) = (0, 0, 0);
        line.1
            .split(" ")
            .map(|x| x.trim())
            .enumerate()
            .for_each(|x| match x.1.parse::<i64>() {
                Ok(num) => match x.0 {
                    0 => values.0 = num,
                    1 => values.1 = num,
                    2 => values.2 = num,
                    _ => panic!("Unknown value"),
                },
                Err(_) => panic!("Unable to parse number {}", x.1),
            });
        match current_type {
            MapType::Soil => {
                seed_to_soil.add_map(Map::new(values.1, values.0, values.2));
            }
            MapType::Fertilizer => {
                soil_to_fertilizer.add_map(Map::new(values.1, values.0, values.2));
            }
            MapType::Water => {
                fertilizer_to_water.add_map(Map::new(values.1, values.0, values.2));
            }
            MapType::Light => {
                water_to_light.add_map(Map::new(values.1, values.0, values.2));
            }
            MapType::Temperature => {
                light_to_temperature.add_map(Map::new(values.1, values.0, values.2));
            }
            MapType::Humidity => {
                temperature_to_humidity.add_map(Map::new(values.1, values.0, values.2));
            }
            MapType::Location => {
                humidity_to_location.add_map(Map::new(values.1, values.0, values.2));
            }
        }
    });

    let mut min_location = -1;
    for seed in seeds {
        let mut current_seed = seed;
        current_seed.set_soil(seed_to_soil.map(current_seed.seed));
        current_seed.set_fertilizer(soil_to_fertilizer.map(current_seed.soil));
        current_seed.set_water(fertilizer_to_water.map(current_seed.fertilizer));
        current_seed.set_light(water_to_light.map(current_seed.water));
        current_seed.set_temperature(light_to_temperature.map(current_seed.light));
        current_seed.set_humidity(temperature_to_humidity.map(current_seed.temperature));
        current_seed.set_location(humidity_to_location.map(current_seed.humidity));
        if min_location == -1 || current_seed.location < min_location {
            min_location = current_seed.location;
        }
    }

    println!("Lowest Seed Location: {}", min_location);
}

fn parse_seeds(contents: &str) -> Vec<SeedData> {
    let mut result = Vec::new();
    contents.lines().enumerate().for_each(|line| {
        if line.0 > 0 {
            return;
        }
        let seed_row = line
            .1
            .split("seeds: ")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        let seeds = seed_row[1]
            .split(" ")
            .map(|x| x.trim())
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        seeds.iter().for_each(|seed| {
            result.push(SeedData::new(*seed));
            // println!("{}", seed);
        });
    });
    result
}
