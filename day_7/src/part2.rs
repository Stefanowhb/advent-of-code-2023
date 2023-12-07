use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Deref;

const CARD_LABELS: [Card; 13] = [
    Card('J'),
    Card('2'),
    Card('3'),
    Card('4'),
    Card('5'),
    Card('6'),
    Card('7'),
    Card('8'),
    Card('9'),
    Card('T'),
    Card('Q'),
    Card('K'),
    Card('A')
];

const JOKER_CARD: Card = CARD_LABELS[0];
const NONE_CARD: Card = Card::new(' ');

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    fn weight(&self) -> usize {
        let a = &""[..];
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }

    fn determine_hand(hand: &Hand) -> Self {
        let mut cards = hand.cards.to_vec();

        //normalize
        cards.sort_by(|c1, c2| c2.weight().cmp(&c1.weight()));

        let mut largest_collection = 0usize;
        let mut collection_count = 0usize;
        let mut count = 0usize;
        let mut last_card = NONE_CARD;

        for card in &cards {
            let mut card = card;

            if *card == JOKER_CARD && last_card == NONE_CARD { // we can exit out early because it's all Jokers
                largest_collection = 5;
                collection_count = 1;

                break;
            } else if *card == JOKER_CARD {
                largest_collection += 1;

                continue;
            }

            if last_card == *card {
                count += 1;
            } else {
                last_card = *card;
                collection_count += 1;
                count = 1;
            }

            if count > largest_collection {
                largest_collection = count;
            }
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

impl Deref for Card {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Card {
    const fn new(label: char) -> Self {
        Self(label)
    }

    fn weight(&self) -> usize {
        CARD_LABELS.iter().position(|c| c == self).expect("Invalid card") + 1
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
        let h1_type_weight = HandType::determine_hand(h1).weight();
        let h2_type_weight = HandType::determine_hand(h2).weight();
        let type_cmp = h1_type_weight.cmp(&h2_type_weight);

        match type_cmp {
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

                println!("{:?}, {:?}", h1.cards, h2.cards);
                return Ordering::Equal
            }
            cmp => cmp
        }
    });

    let mut result = 0usize;

    for (rank, hand) in hands.iter().enumerate() {
        result += hand.bid * (rank + 1);
    }

    println!("Part 2 result: {result}");

    Ok(())
}
