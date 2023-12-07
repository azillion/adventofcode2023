use std::{collections::hash_map, fs};

const DEFAULT_FILE_PATH: &str = "./input.txt";

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Hands {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, Hash)]
enum Cards {
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
}

impl Cards {
    fn new(card: &str) -> Self {
        let card = match card {
            "2" => Cards::Two,
            "3" => Cards::Three,
            "4" => Cards::Four,
            "5" => Cards::Five,
            "6" => Cards::Six,
            "7" => Cards::Seven,
            "8" => Cards::Eight,
            "9" => Cards::Nine,
            "T" => Cards::Ten,
            "J" => Cards::Jack,
            "Q" => Cards::Queen,
            "K" => Cards::King,
            "A" => Cards::Ace,
            _ => panic!("Invalid card"),
        };
        card
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Cards>,
    hand: Hands,
}

impl Hand {
    fn new(cards: Vec<Cards>) -> Self {
        Self {
            cards,
            hand: Hands::HighCard,
        }
    }

    fn calculate_hand(&mut self) {
        let mut cards = self.cards.clone();
        cards.sort();
        let mut hand = Hands::HighCard;
        let mut pair = 0;
        let mut three = 0;
        let mut four = 0;
        let mut five = 0;

        let mut hash_map = hash_map::HashMap::new();

        for card in cards {
            if hash_map.contains_key(&card) {
                let count = hash_map.get(&card).unwrap();
                hash_map.insert(card, count + 1);
            } else {
                hash_map.insert(card, 1);
            }
        }
        for (_, count) in hash_map {
            if count == 2 {
                pair += 1;
            } else if count == 3 {
                three += 1;
            } else if count == 4 {
                four += 1;
            } else if count == 5 {
                five += 1;
            }
        }
        if five == 1 {
            hand = Hands::FiveOfAKind;
        } else if four == 1 {
            hand = Hands::FourOfAKind;
        } else if three == 1 && pair == 1 {
            hand = Hands::FullHouse;
        } else if three == 1 {
            hand = Hands::ThreeOfAKind;
        } else if pair == 2 {
            hand = Hands::TwoPair;
        } else if pair == 1 {
            hand = Hands::OnePair;
        }
        self.hand = hand;
    }

    fn cmp_cards(&self, other: &Self) -> std::cmp::Ordering {
        let cards = self.cards.clone();
        let other_cards = other.cards.clone();
        for (card, other_card) in cards.iter().zip(other_cards.iter()) {
            if card > other_card {
                return std::cmp::Ordering::Greater;
            } else if card < other_card {
                return std::cmp::Ordering::Less;
            } else {
                continue;
            }
        }
        std::cmp::Ordering::Equal
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand > other.hand {
            return std::cmp::Ordering::Greater;
        } else if self.hand < other.hand {
            return std::cmp::Ordering::Less;
        } else {
            return self.cmp_cards(other);
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

#[derive(Debug)]
struct Round {
    hand: Hand,
    bid: i64,
}

impl Round {
    fn new(round: &str) -> Self {
        let mut cards = Vec::new();
        let round = round.split_whitespace().collect::<Vec<&str>>();
        for c in round[0].split("") {
            let c = c.trim();
            if c == "" {
                continue;
            }
            cards.push(Cards::new(c));
        }
        let bid = round[1].trim().parse::<i64>().unwrap();
        let mut hand = Hand::new(cards);
        hand.calculate_hand();
        Self { hand, bid }
    }
}

fn main() {
    let start_time = std::time::Instant::now();
    let is_test = false;
    let input = get_input(is_test);
    let mut rounds = Vec::new();
    for round in input.lines() {
        rounds.push(Round::new(round));
    }
    rounds.sort_by(|a, b| a.hand.cmp(&b.hand));

    let sum = rounds
        .iter()
        .enumerate()
        .map(|(i, round)| round.bid * (i as i64 + 1))
        .sum::<i64>();
    println!("Sum: {}", sum);
    println!("Elapsed time: {:.2?}", start_time.elapsed());
}

fn get_input(is_test: bool) -> String {
    if is_test {
        return "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string();
    }
    fs::read_to_string(DEFAULT_FILE_PATH).expect("Should have been able to read the file")
}
