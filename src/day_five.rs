use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

const INPUT_PATH: &'static str = "data/day_five/puzzle_input.txt";
// const INPUT_PATH: &'static str = "data/day_five/test_input.txt";

// TODO: move this into its own mod and use it everywhere
fn get_file_lines<P>(filename: P) -> Lines<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Can't read input file. Something ain't right.");
    return BufReader::new(file).lines();
}

#[derive(Debug)]
struct Mapping {
    destination: i64,
    // the range is exclusive!
    range: i64,
    src: i64,
}
impl Mapping {
    fn new(destination: i64, src: i64, range: i64) -> Self {
        return Mapping {
            destination,
            src,
            range,
        };
    }
}

// ideas for algorithm:
// to determine mapping value:
// 1. if target < src, continue
// 2. else if target - src  < range, we found the mapping. (dest - src) + target for next mapping
//    target
// if mapping not found after all elements are exhausted, next mapping target = target and we
// continue
pub fn part_one() {
    let mut lines = get_file_lines(INPUT_PATH);

    // parse seeds outta first line
    let seeds: Vec<i64> = lines
        .next()
        .expect("Can't read first line of file.")
        .expect("Can't read content of first line")
        .split_once(": ")
        .expect("Can't split on a colon.")
        .1
        .split(' ')
        .map(|s| s.parse::<i64>().expect("Can't parse seed num as u32."))
        .collect();

    let mut mappings: Vec<Vec<Mapping>> = Vec::new();
    let mut map_len = 0;
    for line in lines {
        let content = line.expect("can't parse line!");

        if content.is_empty() {
            continue;
        }

        // prepare for new mapping
        if content.ends_with("map:") {
            mappings.push(Vec::new());
            map_len += 1;
            continue;
        }

        let row: Vec<i64> = content
            .split(' ')
            .map(|s| s.parse::<i64>().expect("Can't parse number to u32"))
            .collect();

        // push into the last item in the vec
        mappings
            .last_mut()
            .expect("Mapping doesn't have a 'last' item...")
            .push(Mapping::new(row[0], row[1], row[2]));
    }

    let mut lowest_location = None;
    for &seed in seeds.iter() {
        let mut src_value = seed;
        let mut i = 0;
        while i < map_len {
            let current_maps = &mappings[i];
            for mapping in current_maps.iter() {
                if src_value < mapping.src {
                    continue;
                }

                // we've found it!
                // these unwraps aren't great, but keeping them here just to get this going
                if (src_value - mapping.src) < mapping.range {
                    src_value = (mapping.destination - mapping.src) + src_value;
                    break;
                }
            }

            i += 1;
        }

        if lowest_location.is_none() || lowest_location.unwrap() > src_value {
            lowest_location = Some(src_value);
        }
    }

    let answer = lowest_location.expect("Couldn't determine lowest location");
    assert_eq!(answer, 251346198);
    println!("Day 5 Part 1: {}", answer);
}

pub fn part_two() {}
