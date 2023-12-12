use crate::util::get_file_lines;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};

const INPUT_PATH: &'static str = "data/day_seven/puzzle_input.txt";
// const INPUT_PATH: &'static str = "data/day_seven/test_input.txt";

lazy_static! {
    static ref RANKINGS: HashMap<char, u16> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
}

#[derive(Debug, Eq)]
struct Hand {
    bid: usize,
    rank: u8,
    card_values: Vec<u16>,
}
impl Hand {
    fn new(bid: usize, hand_str: &str) -> Self {
        let mut map = BTreeMap::new();
        let mut card_values = Vec::new();
        for c in hand_str.chars() {
            let value = RANKINGS.get(&c).expect("Can't get mapping for card!");
            map.entry(value)
                .and_modify(|count| *count += 1usize)
                .or_insert(1usize);
            card_values.push(*value);
        }

        let key_len = map.keys().len();
        let rank = match map.values().max().expect("Can't get max from hand!") {
            5 => 7,
            4 => 6,
            3 => {
                if key_len == 2 {
                    5
                } else {
                    4
                }
            }
            2 => {
                if key_len == 3 {
                    3
                } else {
                    2
                }
            }
            _ => 1,
        };

        return Self {
            bid,
            rank,
            card_values,
        };
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.rank != other.rank {
            return false;
        }

        for (i, &val) in self.card_values.iter().enumerate() {
            if val != other.card_values[i] {
                return false;
            }
        }

        return true;
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.rank.cmp(&other.rank);

        if ord == Ordering::Equal {
            for (i, &val) in self.card_values.iter().enumerate() {
                let other_val = other.card_values[i];
                if val != other_val {
                    return val.cmp(&other_val);
                }
            }
        }

        return ord;
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

pub fn part_one() {
    let lines = get_file_lines(INPUT_PATH);

    let mut all_hand_results: Vec<Hand> = Vec::new();

    for line in lines {
        let content = line.expect("File line can't be read");
        let (hand, bid) = content
            .split_once(' ')
            .map(|h| {
                (
                    h.0,
                    h.1.parse::<usize>().expect("Can't parse bid into usize"),
                )
            })
            .expect("Can't split hand line on space");

        all_hand_results.push(Hand::new(bid, hand));
    }

    let mut sum = 0;
    all_hand_results.sort();
    for (i, hand) in all_hand_results.iter().enumerate() {
        sum += (i + 1) * hand.bid;
    }

    assert_eq!(sum, 250898830);
    println!("Day 7 Part 1: {}", sum);
}

pub fn part_two() {}
