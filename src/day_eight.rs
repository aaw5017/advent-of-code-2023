use crate::util::get_file_lines;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
};

const INPUT_PATH: &'static str = "data/day_eight/puzzle_input.txt";
// const INPUT_PATH: &'static str = "data/day_eight/test_input.txt";
// const INPUT_PATH: &'static str = "data/day_eight/test_input_2.txt";
const TARGET_KEY: &'static str = "ZZZ";

fn get_instructions(lines: &mut Lines<BufReader<File>>) -> Vec<char> {
    let vec = lines
        .next()
        .expect("Can't read first line of file")
        .expect("Can't get content from first line")
        .chars()
        .collect::<Vec<_>>();

    // remove the empty line
    lines.next();

    return vec;
}

fn map_part_one(lines: Lines<BufReader<File>>) -> HashMap<String, (String, String)> {
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let content = line.expect("Line can't be read");

        let (key, right) = content
            .split_once(" = ")
            .expect("Can't split line on ' = '");

        let value_tuple = right
            .replace('(', "")
            .replace(')', "")
            .split_once(", ")
            .map(|(l, r)| (l.to_string(), r.to_string()))
            .expect("Can't map RHS into tuple");

        map.insert(key.to_string(), value_tuple);
    }

    return map;
}

pub fn part_one() {
    let mut lines = get_file_lines(INPUT_PATH);
    let instructions = get_instructions(&mut lines);
    let map = map_part_one(lines);

    let mut target_found = false;
    let mut num_iterations = 0;
    let mut current_key = "AAA";

    while !target_found {
        for &inst in instructions.iter() {
            let (left, right) = match map.get(current_key) {
                Some(v) => v,
                None => {
                    eprintln!("Can't get value for key {}", current_key);
                    std::process::exit(1);
                }
            };

            current_key = if inst == 'L' { left } else { right };
            num_iterations += 1;

            if current_key == TARGET_KEY {
                target_found = true;
                break;
            }
        }
    }

    assert_eq!(num_iterations, 11567);
    println!("Day 8 Part 1: {}", num_iterations);
}

fn fill_paths<T>(
    current: T,
    map: &HashMap<String, (String, String)>,
    instruction: char,
) -> Vec<&str>
where
    T: IntoIterator,
    T::Item: Into<String> + std::fmt::Display,
{
    let ret = current
        .into_iter()
        .map(|key| {
            let (left, right) = match map.get(&key.into()) {
                Some(v) => v,
                None => {
                    eprintln!("Can't get value for key");
                    std::process::exit(1);
                }
            };

            if instruction == 'L' {
                left.as_str()
            } else {
                right.as_str()
            }
        })
        .collect();

    return ret;
}

fn map_part_two<'a>(
    lines: Lines<BufReader<File>>,
) -> (HashMap<String, (String, String)>, Vec<String>) {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut starting_keys = Vec::new();

    for line in lines {
        let content = line.expect("Line can't be read");

        let (key, right) = content
            .split_once(" = ")
            .expect("Can't split line on ' = '");

        let key_string = key.to_string();
        if key_string.ends_with('A') {
            starting_keys.push(key_string.to_string());
        }

        let value_tuple = right
            .replace('(', "")
            .replace(')', "")
            .split_once(", ")
            .map(|(l, r)| (l.to_string(), r.to_string()))
            .expect("Can't map RHS into tuple");

        map.insert(key_string, value_tuple);
    }

    return (map, starting_keys);
}

// Used spoilers on this one
// I had the correct thought process, but didn't recognize the LCM (least common multiple) pattern
// Shout out to https://www.youtube.com/@WilliamYFeng
pub fn part_two() {
    let mut lines = get_file_lines(INPUT_PATH);
    let instructions = get_instructions(&mut lines);
    let (map, starting_keys) = map_part_two(lines);

    let mut all_iterations = Vec::new();
    let mut current_keys = starting_keys.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    for key in current_keys.iter_mut() {
        let mut target_found = false;
        let mut num_iterations = 0;
        while !target_found {
            for &inst in instructions.iter() {
                num_iterations += 1;

                let (left, right) = match map.get(*key) {
                    Some(v) => v,
                    None => {
                        eprintln!("Can't get value for key");
                        std::process::exit(1);
                    }
                };

                *key = if inst == 'L' {
                    left.as_str()
                } else {
                    right.as_str()
                };

                if key.ends_with('Z') {
                    target_found = true;
                    all_iterations.push(num_iterations);
                    break;
                }
            }
        }
    }

    println!("Day 8 Part 2: {:?}", all_iterations);
    println!("Plug this ^ into an online LCM calc for the answer (Can't figure out how to use LCM functions on vectors yet)");
}
