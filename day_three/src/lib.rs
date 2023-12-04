use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &'static str = "day_three/data/puzzle_input.txt";
// const INPUT_PATH: &'static str = "day_three/data/test_input.txt";
const REGEX_PATTERN: &'static str = "(?<number>\\d+)|(?<symbol>[^\\.\\n])";

struct NumberItem {
    end: usize,
    line_num: usize,
    start: usize,
    value: i32,
}

struct SymbolItem {
    index: usize,
    line_num: usize,
}

pub fn part_one() {
    let file = File::open(INPUT_PATH).expect("Can't read input file. Something ain't right.");
    let lines = BufReader::new(file).lines();

    let regex = Regex::new(REGEX_PATTERN).expect("Can't create regex. Something ain't right.");
    let mut numbers: Vec<NumberItem> = Vec::new();
    let mut symbols: Vec<SymbolItem> = Vec::new();

    let mut line_num = 0;
    for line in lines {
        let content = line.expect("Can't get content from line. Boo.");
        let captures = regex.captures_iter(&content).collect::<Vec<_>>();
        for cap in captures.iter() {
            if let Some(number_match) = cap.name("number") {
                let item = NumberItem {
                    end: number_match.end(),
                    line_num,
                    start: number_match.start(),
                    value: number_match.as_str().parse::<i32>().unwrap_or(0),
                };

                numbers.push(item);
            }
            if let Some(symbol_match) = cap.name("symbol") {
                let item = SymbolItem {
                    index: symbol_match.start(),
                    line_num,
                };
                symbols.push(item);
            }
        }

        line_num = line_num + 1;
    }

    let mut sum = 0;
    for num in numbers.iter() {
        let line_num = num.line_num;
        let range_start = if num.start > 0 {
            num.start - 1
        } else {
            num.start
        };
        let target_range = range_start..=num.end;
        let line_below = line_num + 1;
        let line_above = if line_num > 0 { line_num - 1 } else { line_num };
        let line_range = line_above..=line_below;

        for sym in symbols.iter() {
            if !line_range.contains(&sym.line_num) {
                continue;
            }
            if target_range.contains(&sym.index) {
                sum = sum + num.value;
            }
        }
    }

    assert_eq!(sum, 544664);
    println!("Day 3 Part 1: {}", sum);
}
