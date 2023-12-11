use std::collections::{BTreeMap, HashMap};

use crate::util::get_file_lines;

// const INPUT_PATH: &'static str = "data/day_seven/puzzle_input.txt";
// const INPUT_PATH: &'static str = "data/day_seven/test_input.txt";
const INPUT_PATH: &'static str = "data/day_seven/test_input_2.txt";

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

type MyType = BTreeMap<u16, u16>;

struct Hand {
    singles: (usize, usize),
    pairs: (usize, usize),
    threes: (usize, usize),
    fours: (usize, usize),
    fives: (usize, usize),
}

// impl Hand {
//     fn new() -> Self {}
// }

pub fn part_one() {
    let lines = get_file_lines(INPUT_PATH);

    let mut num_hands = 0;
    let mut all_hand_results: Vec<MyType> = Vec::new();

    for line in lines {
        let mut hand_results: MyType = MyType::new();
        let content = line.expect("File line can't be read");
        let (hand, bid) = content
            .split_once(' ')
            .map(|h| (h.0, h.1.parse::<u16>().expect("Can't parse bid into u16")))
            .expect("Can't split hand line on space");

        let mut rankings = hand.chars().collect::<Vec<_>>();
        rankings.sort_by(|a, b| {
            let card_a_rank = RANKINGS.get(a).unwrap_or(&0);
            let card_b_rank = RANKINGS.get(b).unwrap_or(&0);
        });
        println!("{:#?}", rankings);

        // for card in hand.chars().into_iter() {
        //     let Some(ranking) = RANKINGS.get(&card) else {
        //         continue;
        //     };

        //     hand_results
        //         .entry(*ranking)
        //         .and_modify(|count| *count += 1)
        //         .or_insert(1);
        // }

        // all_hand_results.push(hand_results);

        // num_hands += 1;
    }

    // println!("{:#?}", all_hand_results);
}

pub fn part_two() {}
