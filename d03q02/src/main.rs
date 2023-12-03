use std::fs;

const DEFAULT_FILE_PATH: &str = "./input.txt";
// const VOID_CHAR: char = '.';
const GEAR_CHAR: char = '*';

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(index: i32, line: i32) -> Position {
        Position { x: index, y: line }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Number {
    width: i32,
    value: i32,
    position: Position,
    bounds: Bounds,
}

impl Number {
    fn new(width: i32, value: i32, position: Position) -> Number {
        let pos = position.clone();
        Number {
            width,
            value,
            position,
            bounds: Bounds::new(pos.x, pos.x + width - 1, pos.y, pos.y),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Symbol {
    value: char,
    position: Position,
    bounds: Bounds,
}

impl Symbol {
    fn new(value: char, position: Position) -> Symbol {
        let pos = position.clone();
        Symbol {
            value,
            position,
            bounds: Bounds::new(pos.x - 1, pos.x + 1, pos.y - 1, pos.y + 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Data {
    fn new() -> Data {
        Data {
            numbers: Vec::new(),
            symbols: Vec::new(),
        }
    }

    fn add_number(&mut self, number: Number) {
        self.numbers.push(number);
    }

    fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    fn merge(&mut self, other: &Data) {
        self.numbers.extend(other.numbers.iter().cloned());
        self.symbols.extend(other.symbols.iter().cloned());
    }

    #[allow(dead_code)]
    fn display(&self) {
        self.numbers.iter().for_each(|number| {
            println!(
                "Number: {} and Bounds ({}, {}, {}, {})",
                number.value,
                number.bounds.min_x,
                number.bounds.max_x,
                number.bounds.min_y,
                number.bounds.max_y
            );
        });
        self.symbols.iter().for_each(|symbol| {
            println!(
                "Symbol: {} and Bounds ({}, {}, {}, {})",
                symbol.value,
                symbol.bounds.min_x,
                symbol.bounds.max_x,
                symbol.bounds.min_y,
                symbol.bounds.max_y
            );
        });
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Bounds {
    fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Bounds {
        Bounds {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn intersects(&self, other: &Bounds) -> bool {
        self.min_x <= other.max_x
            && self.max_x >= other.min_x
            && self.min_y <= other.max_y
            && self.max_y >= other.min_y
    }
}

fn main() {
    let contents =
        fs::read_to_string(DEFAULT_FILE_PATH).expect("Should have been able to read the file");

    let mut sum = 0;
    let mut data = Data::new();
    contents.lines().enumerate().for_each(|line| {
        let line_data = parse_line(line.1, line.0 as i32);
        data.merge(&line_data);
    });

    // data.display();

    data.symbols.iter().for_each(|symbol| {
        let mut number_count = 0;
        let mut number_product = 1;
        data.numbers.iter().for_each(|number| {
            if number.bounds.intersects(&symbol.bounds) {
                number_count += 1;
                number_product *= number.value;
            }
        });
        if number_count >= 2 {
            sum += number_product;
        }
    });

    println!("Sum: {}", sum);
}

fn parse_line(line: &str, line_number: i32) -> Data {
    let mut data = Data::new();
    let mut index = 0;
    let mut number_width = 0;
    let mut number_value = 0;

    for c in line.chars() {
        if c.is_digit(10) {
            number_width += 1;
            number_value = number_value * 10 + c.to_digit(10).unwrap() as i32;
        } else {
            if number_width > 0 {
                data.add_number(Number::new(
                    number_width,
                    number_value,
                    Position::new(index - number_width, line_number),
                ));
                number_width = 0;
                number_value = 0;
            }
            if c == GEAR_CHAR {
                data.add_symbol(Symbol::new(c, Position::new(index, line_number)));
            }
        }

        index += 1;
    }

    if number_width > 0 {
        data.add_number(Number::new(
            number_width,
            number_value,
            Position::new(index - number_width, line_number),
        ));
    }

    data
}
