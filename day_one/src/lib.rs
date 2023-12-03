use lazy_static::lazy_static;
use regex::{Matches, Regex, RegexSet};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

lazy_static! {
    static ref PART_TWO_MAP: HashMap<&'static str, &'static str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
}

const INPUT_PATH: &'static str = "day_one/data/puzzle_input.txt";
// const INPUT_PATH: &'static str = "day_one/data/test_input.txt";
const PART_ONE_REGEX: &'static str = "\\d";
const PART_TWO_REGEX: [&'static str; 10] = [
    "\\d", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

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
    let regex = Regex::new(PART_ONE_REGEX).expect("Regex::new blew up. Something ain't right.");

    for line in lines {
        let content = line.unwrap_or(String::from(""));
        let matches: Vec<_> = regex.find_iter(&content).map(|m| m.as_str()).collect();
        if let Some(first) = matches.first() {
            // last == first if only one element in vector, so unwrapping is safe here
            let last = matches.last().unwrap();
            let mut num_str = String::from(*first);
            num_str.push_str(*last);

            if let Ok(parsed) = num_str.parse::<i32>() {
                total += parsed;
            }
        }
    }

    assert_eq!(total, 53974);
    println!("Day 1 Part 1: {:?}", total);
}

#[derive(Debug)]
struct RegexInfo {
    start: usize,
    matched: String,
}

pub fn part_two() {
    let mut total: i32 = 0;
    let lines = read_lines(INPUT_PATH);
    let set = RegexSet::new(PART_TWO_REGEX).expect("Regex::new blew up. Something ain't right.");
    let regexes: Vec<_> = set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();
    let mut matching_patterns: Vec<RegexInfo> = Vec::new();

    for line in lines {
        let content = line.unwrap_or(String::from(""));

        for regex in regexes.iter() {
            let matches: Matches = regex.find_iter(&content).into_iter();
            for m in matches {
                let m_str = m.as_str();
                let num_str = match PART_TWO_MAP.get(m_str) {
                    Some(found) => found,
                    None => m_str,
                };
                matching_patterns.push(RegexInfo {
                    start: m.start(),
                    matched: num_str.to_owned(),
                });
            }
        }

        matching_patterns.sort_by(|a, b| a.start.cmp(&b.start));
        if let Some(first) = matching_patterns.first() {
            // last == first if only one element in vector, so unwrapping is safe here
            let last = matching_patterns.last().unwrap();
            let mut num_str = first.matched.clone();
            num_str.push_str(&last.matched);

            if let Ok(parsed) = num_str.parse::<i32>() {
                total += parsed;
            }
        }

        matching_patterns.clear();
    }

    assert_eq!(total, 52840);
    println!("Day 1 Part 2: {:?}", total);
}
