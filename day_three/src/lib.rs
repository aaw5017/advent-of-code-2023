use regex::{Captures, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Lines};
use std::path::Path;
use std::slice::SliceIndex;

// const INPUT_PATH: &'static str = "day_three/data/puzzle_input.txt";
const INPUT_PATH: &'static str = "day_three/data/test_input.txt";
const REGEX_PATTERN: &'static str = "(?<number>\\d+)|(?<symbol>[^\\.\\n])";
const DOT_CHAR: char = '.';

fn is_target_symbol(symbol: &char) -> bool {
    if symbol == &DOT_CHAR {
        return false;
    }
    if symbol.is_digit(10) {
        return false;
    }
    return true;
}

fn get_last_number_index(arr: &Vec<Vec<char>>, row_num: usize, col_num: usize) -> usize {
    let current_row = &arr[row_num];

    let mut i = col_num + 1;
    while let Some(el) = current_row.get(i) {
        if el.is_digit(10) {
            i = i + 1;
            continue;
        }

        break;
    }
    return i;
}

pub fn part_one_old() {
    // Initial / naive approach idea
    // 1. open the file
    // 2. read 2 lines in to the file at a time
    // 3. Determine adjacent points within smaller set until whole file has been read

    // further ideas:
    // 1. if number found, keep walking until you hit a non-number
    // 2. starting at current index, determine adjacency
    // 3. walk index-1 for all digits until adjacency is found

    let f = BufReader::new(File::open(INPUT_PATH).unwrap());
    let arr = f
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // i is row iterator
    let mut i = 0;
    let arr_len = arr.len();

    while i < arr_len {
        let row = &arr[i];

        // js is col iterator
        let mut j = 0;
        let row_len = row.len();
        while j < row_len {
            let el = arr[i][j];
            if el.is_digit(10) {
                let new_j = get_last_number_index(&arr, i, j);
                let slice = &arr[i][j..new_j];
                let mut num_str = String::from("");
                for char in slice.iter() {
                    num_str.push(*char);
                }

                println!("{}", num_str);
                j = new_j;
                continue;
            }

            j = j + 1;
        }

        i = i + 1;
    }
}

pub fn part_one() {
    let f = BufReader::new(File::open(INPUT_PATH).unwrap());
    let mut lines = f.lines();

    let mut content_one = String::new();
    let mut content_two = String::new();
    while let Some(line_one) = lines.next() {
        if let Some(line_two) = lines.next() {
            content_one = line_one.expect("should read line_one");
            content_two = line_two.expect("should read line_two");
            println!("{}", content_one);
            println!("{}", content_two);

            continue;
        }
        break;
    }
}
