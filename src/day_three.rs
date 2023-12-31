use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_PATH: &'static str = "data/day_three/puzzle_input.txt";
// const INPUT_PATH: &'static str = "data/day_three/test_input.txt";
const PART_ONE_REGEX_PATTERN: &'static str = "(?<number>\\d+)|(?<symbol>[^\\.\\n])";
const PART_TWO_REGEX_PATTERN: &'static str = "(?<number>\\d+)|(?<symbol>\\*)";

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

fn fill_vectors<P>(pattern: &str, filename: P) -> (Vec<NumberItem>, Vec<SymbolItem>)
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Can't read input file. Something ain't right.");
    let lines = BufReader::new(file).lines();

    let regex = Regex::new(pattern).expect("Can't create regex. Something ain't right.");
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

    return (numbers, symbols);
}

pub fn part_one() {
    let (numbers, symbols) = fill_vectors(PART_ONE_REGEX_PATTERN, INPUT_PATH);

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

pub fn part_two() {
    let (numbers, symbols) = fill_vectors(PART_TWO_REGEX_PATTERN, INPUT_PATH);
    let mut adjacents: Vec<i32> = Vec::new();
    let mut sum = 0;

    for sym in symbols.iter() {
        let line_num = sym.line_num;
        let line_below = line_num + 1;
        let line_above = if line_num > 0 { line_num - 1 } else { line_num };
        let line_range = line_above..=line_below;
        let range_start = if sym.index > 0 {
            sym.index - 1
        } else {
            sym.index
        };
        let target_range = range_start..=(sym.index + 1);

        for num in numbers.iter() {
            if !line_range.contains(&num.line_num) {
                continue;
            }
            if target_range.contains(&num.start) || target_range.contains(&(num.end - 1)) {
                adjacents.push(num.value);
            }
        }

        if adjacents.len() == 2 {
            let mut product = 1;
            for num in adjacents.iter() {
                product *= num;
            }
            sum += product;
        }

        adjacents.clear();
    }

    assert_eq!(sum, 84495585);
    println!("Day 3 Part 2: {}", sum);
}
