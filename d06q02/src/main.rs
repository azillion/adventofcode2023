#[derive(Debug)]
struct Entry {
    time: u64,
    charge: u64,
}

impl Entry {
    fn new(time: u64, charge: u64) -> Self {
        Self { time, charge }
    }

    fn acceleration(&self) -> u64 {
        self.charge
    }

    fn calculate_distance(&self) -> u64 {
        let mut distance = 0;
        for _ in self.charge..self.time {
            distance += self.acceleration();
        }
        distance
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn new(time: u64, record: u64) -> Self {
        Self { time, record }
    }

    fn calculate_solutions(&self) -> u64 {
        let mut entries = Vec::new();
        for i in 0..self.time + 1 {
            entries.push(Entry::new(self.time, i));
        }
        entries
            .iter()
            .filter(|e| e.calculate_distance() > self.record)
            .count() as u64
    }
}

fn main() {
    let start_time = std::time::Instant::now();

    let races = get_races(false);

    let total_solutions = races
        .iter()
        .map(|r| r.calculate_solutions())
        .product::<u64>();

    println!("Total solutions: {}", total_solutions);
    let elapsed_time = start_time.elapsed();
    println!("Total Execution Time: {:?}", elapsed_time);
}

fn get_races(is_test: bool) -> Vec<Race> {
    let mut races = Vec::new();
    if is_test {
        races.push(Race::new(71530, 940200));
    } else {
        races.push(Race::new(56977793, 499221010971440));
    }
    races
}
