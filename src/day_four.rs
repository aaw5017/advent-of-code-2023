use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

const INPUT_PATH: &'static str = "data/day_four/puzzle_input.txt";
// const INPUT_PATH: &'static str = "data/day_four/test_input.txt";

fn get_file_lines<P>(filename: P) -> Lines<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Can't read input file. Something ain't right.");
    return BufReader::new(file).lines();
}

fn get_game_matches(game_line: Result<String, std::io::Error>) -> usize {
    let game = game_line.expect("Can't read file line. Something ain't right.");
    let detail_split = game.split(": ").last().expect("Can't split game line.");
    let game_details = detail_split.split(" | ").collect::<Vec<_>>();
    let wins = game_details[0].split(' ');
    let ours = game_details[1].split(' ');

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
    return inter.len();
}

pub fn part_one() {
    let lines = get_file_lines(INPUT_PATH);
    let mut sum = 0;

    for line in lines {
        let matches = get_game_matches(line);
        if matches == 0 {
            continue;
        }

        let c: u32 = matches.try_into().expect("can't convert usize into u32...");
        let pow_pow = 2u32.pow(c - 1);
        let result = 1 * pow_pow;

        sum += result;
    }

    assert_eq!(sum, 25010);
    println!("Day 4 Part 1: {}", sum);
}

fn process_game_copies(
    all_games: &[usize],
    game_index: usize,
    num_copies: usize,
    times_processed: usize,
) -> usize {
    let copy_range = (game_index + 1)..=(game_index + num_copies);

    let mut tp = times_processed;
    for i in copy_range {
        let result = all_games[i];
        tp += process_game_copies(all_games, i, result, times_processed);
    }

    return tp;
}

// I can't imagine this is performant, but meh
pub fn part_two() {
    let lines = get_file_lines(INPUT_PATH);
    let mut game_results: Vec<usize> = Vec::new();
    let mut sum = 0;

    for line in lines {
        let matches = get_game_matches(line);
        game_results.push(matches);
    }

    for (i, &result) in game_results.iter().enumerate() {
        sum += process_game_copies(&game_results, i, result, 1);
    }

    assert_eq!(sum, 9924412);
    println!("Day 4 Part 2: {}", sum);
}
