use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const JOKER_CARD: Card = Card::new('J');
const NONE_CARD: Card = Card::new(' ');

#[derive(Debug, Copy, Clone)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl HandType {
    fn weight(&self) -> usize {
        *self as usize
    }

    fn determine(hand: &Hand) -> Self {
        let mut cards = hand.cards.to_vec();
        cards.sort_by(|c1, c2| c2.weight().cmp(&c1.weight()));

        let mut largest_collection = 0;
        let mut collection_count = 0;
        let mut count = 0;
        let mut last_card = NONE_CARD;

        for card in &cards {
            match *card {
                JOKER_CARD if last_card == NONE_CARD => {
                    return HandType::FiveOfAKind;
                },
                JOKER_CARD => largest_collection += 1,
                _ => {
                    if last_card == *card {
                        count += 1;
                    } else {
                        last_card = *card;
                        count = 1;
                        collection_count += 1;
                    }
                }
            }

            largest_collection = largest_collection.max(count);
        }

        match (largest_collection, collection_count) {
            (5, _) => Self::FiveOfAKind,
            (4, _) => Self::FourOfAKind,
            (3, 2) => Self::FullHouse,
            (3, 3) => Self::ThreeOfAKind,
            (2, 3) => Self::TwoPair,
            (2, 4) => Self::OnePair,
            (1, 5) => Self::HighCard,
            (a, b) => panic!("Invalid hand: {a} {b}")
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Card (char);

impl Card {
    const fn new(label: char) -> Self {
        Self(label)
    }

    const fn weight(&self) -> usize {
        match self.0 {
            'J' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => panic!("Invalid card")
        }
    }
}

struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: usize) -> Self {
        Self {
            cards,
            bid,
        }
    }
}

fn parse_input(file: File) -> Result<Vec<Hand>, Box<dyn Error>> {
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|r| r.expect("Failed to read line"))
        .collect();
    let hands: Vec<Hand> = lines.iter()
        .map(|l| {
            let split = l.split_once(" ").expect("Invalid file format");
            let hand_string: Vec<Card> = split.0.chars()
                .map(Card::new)
                .collect();
            let bid = split.1.parse::<usize>().expect("Invalid file format");

            Hand::new(hand_string, bid)
        })
        .collect();

    Ok(hands)
}

pub fn part2() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_7_input.txt")?;
    let mut hands = parse_input(file)?;

    hands.sort_by(|h1, h2| {
        let h1_weight = HandType::determine(h1).weight();
        let h2_weight = HandType::determine(h2).weight();
        let cmp = h1_weight.cmp(&h2_weight);

        match cmp {
            Ordering::Equal => {
                let zip = h1.cards.iter().zip(h2.cards.iter());

                for (c1, c2) in zip {
                    let cmp = c1.weight().cmp(&c2.weight());

                    match cmp {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        _ => {}
                    }
                }

                return Ordering::Equal
            }
            cmp => cmp
        }
    });

    let mut result = 0;

    for (rank, hand) in hands.iter().enumerate() {
        result += hand.bid * (rank + 1);
    }

    println!("Part 2 result: {result}");

    Ok(())
}
