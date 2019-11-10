use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{HashMap, HashSet};

struct Card {
    rank: u8,
    suit: char
}

impl From<&str> for Card {
    fn from(card: &str) -> Self {
        let (rank, suit) = card.split_at(card.len() - 1);
        Self {
            rank: match rank {
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                "A" => 14,
                _ => rank.parse().unwrap()
            },
            suit: match suit {
                "H"|"S"|"D"|"C" => suit.chars().nth(0).unwrap(),
                _ => panic!()
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Hand {
    HighCard(Vec<u8>),
    OnePair(u8, u8),
    TwoPair(u8, u8, u8),
    ThreeOfAKind(u8, u8),
    Straight(u8),
    Flush(u8),
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8)
}

impl From<&str> for Hand {
    fn from(hand: &str) -> Self {
        let cards = {
            let mut tmp = hand
                .split_whitespace()
                .map(|s| Card::from(s))
                .collect::<Vec<Card>>();
            tmp.sort_by_key(|c| c.rank);
            tmp
        };
        let (ranks, counts): (Vec<u8>, Vec<u8>) = {
            let mut tmp = cards
                .iter()
                .fold(HashMap::new(), |mut acc, c| {
                    acc.entry(c.rank)
                       .and_modify(|x| *x += 1)
                       .or_insert(1u8);
                    acc
                })
                .drain()
                .collect::<Vec<(u8, u8)>>();
            tmp.sort_by_key(|&(_, v)| Reverse(v));
            tmp.iter().cloned().unzip()
        };
        let freq_card = ranks[0];
        let high_card = *ranks.iter().max().unwrap();
        let kicker = *ranks[1..].into_iter().max().unwrap();

        match counts[..] {
            [1, 1, 1, 1, 1] => {
                let is_flush: bool = cards
                    .iter()
                    .map(|c| c.suit)
                    .collect::<HashSet<char>>()
                    .len() == 1;
                let is_straight: bool = cards
                    .windows(2)
                    .all(|c| c[0].rank + 1 == c[1].rank);
                let is_low_straight: bool = cards
                    .iter()
                    .map(|c| c.rank)
                    .collect::<Vec<u8>>()[..] == [2, 3, 4, 5, 14];
                match (is_flush, is_straight, is_low_straight) {
                    (false, _, true) => Hand::Straight(5),
                    (true, _, true) => Hand::StraightFlush(5),
                    (true, true, false) => Hand::StraightFlush(high_card),
                    (true, false, false) => Hand::Flush(high_card),
                    (false, true, false) => Hand::Straight(high_card),
                    (false, false, false) => Hand::HighCard(
                        cards.iter()
                             .map(|c| c.rank)
                             .collect::<Vec<u8>>()
                    ),
                }
            },
            [2, 1, 1, 1] => Hand::OnePair(freq_card, kicker),
            [2, 2, 1] => {
                let high_pair = max(freq_card, ranks[1]);
                let low_pair = min(freq_card, ranks[1]);
                let last_card = ranks[2];
                Hand::TwoPair(high_pair, low_pair, last_card)
            },
            [3, 1, 1] => Hand::ThreeOfAKind(freq_card, kicker),
            [3, 2] => Hand::FullHouse(freq_card, kicker),
            [4, 1] => Hand::FourOfAKind(freq_card, kicker),
            _ => panic!()
        }
    }
}

pub fn winning_hands<'a>(input: &[&'a str]) -> Option<Vec<&'a str>> {
    match input.is_empty() {
        true => None,
        false => Some({
            let mut hands = input.iter()
                                 .map(|&s| (Hand::from(s), s))
                                 .collect::<Vec<(Hand, &str)>>();
            hands.sort_by(|(a, _), (b, _)| {
                a.partial_cmp(b)
                 .unwrap_or(Ordering::Equal)
            });
            hands.iter()
                 .filter(|&(h, _)| h == &hands[hands.len()-1].0)
                 .map(|&(_, src)| src)
                 .collect::<Vec<&str>>()
        })
    }
}

