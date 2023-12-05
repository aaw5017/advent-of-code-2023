use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &'static str = "day_four/data/puzzle_input.txt";
// const INPUT_PATH: &'static str = "day_four/data/test_input.txt";

pub fn part_one() {
    let file = File::open(INPUT_PATH).expect("Can't read input file. Something ain't right.");
    let lines = BufReader::new(file).lines();

    let mut sum = 0;

    for line in lines {
        let game = line.expect("Can't read file line. Something ain't right.");
        let game_details = game.split(": ").last().expect("Can't split game line.");
        let wins_and_ours: Vec<&str> = game_details.split(" | ").collect();

        let wins = wins_and_ours[0].split(' ');
        let ours = wins_and_ours[1].split(' ');

        let mut wins_set = BTreeSet::new();
        let mut our_set = BTreeSet::new();

        for win in wins {
            if win.is_empty() {
                continue;
            }
            wins_set.insert(win);
        }
        for our in ours {
            if our.is_empty() {
                continue;
            }
            our_set.insert(our);
        }

        let inter: Vec<_> = wins_set.intersection(&our_set).collect();
        let count = inter.len();

        if count == 0 {
            continue;
        }

        let c: u32 = count.try_into().expect("can't convert usize into u32...");
        let pow_pow = 2u32.pow(c - 1);
        let result = 1 * pow_pow;

        sum += result;
    }

    assert_eq!(sum, 25010);
    println!("Day 4 Part 1: {}", sum);
}
