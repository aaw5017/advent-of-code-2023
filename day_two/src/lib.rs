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

    assert_eq!(total, 2406);
    println!("Day 2 Part 1: {}", total);
}

pub fn part_two() {
    // 1. parse each line as we did in part one
    // 2. Determine the maximum number for red, green, blue (respectively) for each round's handful
    // 3. Once the R,G,B max nums are discovered, multiply them together to find the "power" value for the round
    // 4. Sum the power values for the final answer

    let mut total = 0;
    let lines = read_lines(INPUT_PATH);
    let regex =
        Regex::new(PART_ONE_REGEX).expect("Can't instantiate regex. Something ain't right.");

    for line in lines {
        let game = line.expect("couldn't parse line from file!");
        let game_rounds: Vec<&str> = game.split(": ").collect();

        if let Some(game_details) = game_rounds.last() {
            let handfuls: Vec<&str> = game_details.split("; ").collect();
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            for handful in handfuls.iter() {
                let captures: Vec<Captures> =
                    regex.captures_iter(&handful).map(|c| c).collect::<Vec<_>>();

                for cap in captures.iter() {
                    let (_full, [_num_and_color, num, color]) = cap.extract();

                    let Ok(parsed) = num.parse::<i32>() else {
                        continue;
                    };

                    // naive approach to start...
                    if color == "red" && min_red < parsed {
                        min_red = parsed;
                    } else if color == "green" && min_green < parsed {
                        min_green = parsed;
                    } else if color == "blue" && min_blue < parsed {
                        min_blue = parsed;
                    }
                }
            }

            let power_value = min_red * min_green * min_blue;
            total += power_value;
        }
    }

    assert_eq!(total, 78375);
    println!("Day 2 Part 2: {}", total);
}
