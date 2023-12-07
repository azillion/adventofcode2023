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
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Queen = 12,
    King = 13,
    Ace = 14,
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
            "J" => Cards::Joker,
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

        let mut card_groups = hash_map::HashMap::new();

        for card in cards {
            if card_groups.contains_key(&card) {
                let count = card_groups.get(&card).unwrap();
                card_groups.insert(card, count + 1);
            } else {
                card_groups.insert(card, 1);
            }
        }
        let jokers_count: i32 = card_groups.get(&Cards::Joker).unwrap_or(&0).clone();
        for (card, count) in card_groups.into_iter() {
            if card == Cards::Joker {
                continue;
            }
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

        // joker is a wild card
        match jokers_count {
            0 => {
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
            }
            1 => {
                if four == 1 {
                    hand = Hands::FiveOfAKind;
                } else if three == 1 {
                    hand = Hands::FourOfAKind;
                } else if pair == 2 {
                    hand = Hands::FullHouse;
                } else if pair == 1 {
                    hand = Hands::ThreeOfAKind;
                } else {
                    hand = Hands::OnePair;
                }
            }
            2 => {
                if three == 1 {
                    hand = Hands::FiveOfAKind;
                } else if pair == 1 {
                    hand = Hands::FourOfAKind;
                } else {
                    hand = Hands::ThreeOfAKind;
                }
            }
            3 => {
                if pair == 1 {
                    hand = Hands::FiveOfAKind;
                } else {
                    hand = Hands::FourOfAKind;
                }
            }
            4 => {
                hand = Hands::FiveOfAKind;
            }
            5 => {
                hand = Hands::FiveOfAKind;
            }
            _ => {}
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
