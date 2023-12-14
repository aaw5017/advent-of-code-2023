use crate::util::get_file_lines;
use std::collections::HashMap;

const INPUT_PATH: &'static str = "data/day_eight/puzzle_input.txt";
// const INPUT_PATH: &'static str = "data/day_eight/test_input.txt";
const TARGET_KEY: &'static str = "ZZZ";

pub fn part_one() {
    let mut lines = get_file_lines(INPUT_PATH);

    let instructions = lines
        .next()
        .expect("Can't read first line of file")
        .expect("Can't get content from first line")
        .chars()
        .collect::<Vec<_>>();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let content = line.expect("Line can't be read");

        if content.is_empty() {
            continue;
        }

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

pub fn part_two() {}
