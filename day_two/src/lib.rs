use regex::{Captures, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

const INPUT_PATH: &'static str = "day_two/data/puzzle_input.txt";
// const INPUT_PATH: &'static str = "day_two/data/test_input.txt";
const PART_ONE_REGEX: &'static str = "((\\d+) (\\w+))";
const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

// TODO: move this into its own crate so it can be shared
fn read_lines<P>(filename: P) -> Lines<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Can't read input file. Something ain't right.");
    return BufReader::new(file).lines();
}

pub fn part_one() {
    let mut total: i32 = 0;
    let lines = read_lines(INPUT_PATH);
    let mut game_number = 0;
    let regex =
        Regex::new(PART_ONE_REGEX).expect("Can't instantiate regex. Something ain't right.");

    for line in lines {
        game_number = game_number + 1;

        let game = line.expect("couldn't parse line from file!");
        let game_rounds: Vec<&str> = game.split(": ").collect();

        if let Some(game_details) = game_rounds.last() {
            let handfuls: Vec<&str> = game_details.split("; ").collect();
            let mut is_game_possible = true;

            for handful in handfuls.iter() {
                let captures: Vec<Captures> =
                    regex.captures_iter(&handful).map(|c| c).collect::<Vec<_>>();

                let mut total_red = 0;
                let mut total_green = 0;
                let mut total_blue = 0;

                for cap in captures.iter() {
                    let (_full, [_num_and_color, num, color]) = cap.extract();

                    let Ok(parsed) = num.parse::<i32>() else {
                        continue;
                    };

                    // naive approach to start...
                    if color == "red" {
                        total_red += parsed;
                    } else if color == "green" {
                        total_green += parsed;
                    } else {
                        total_blue += parsed;
                    }
                }

                if total_red > MAX_RED || total_green > MAX_GREEN || total_blue > MAX_BLUE {
                    is_game_possible = false;
                }
            }

            if is_game_possible {
                total += game_number;
            }
        }
    }

    assert_eq!(game_number, 100);
    assert_eq!(total, 2406);
    println!("Day 2 Part 1: {}", total);
}

pub fn part_two() {
    println!("hello from day two part two");
}
