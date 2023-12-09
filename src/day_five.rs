use std::cmp::{max, min};
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
    dest: u64,
    // the range is exclusive!
    range: u64,
    src: u64,
}
impl Mapping {
    fn new(dest: u64, src: u64, range: u64) -> Self {
        return Mapping { dest, src, range };
    }
}

fn get_mappings(lines: Lines<BufReader<File>>) -> (Vec<Vec<Mapping>>, usize) {
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

        let row: Vec<u64> = content
            .split(' ')
            .map(|s| s.parse::<u64>().expect("Can't parse number to u32"))
            .collect();

        // push into the last item in the vec
        mappings
            .last_mut()
            .expect("Mapping doesn't have a 'last' item...")
            .push(Mapping::new(row[0], row[1], row[2]));
    }

    return (mappings, map_len);
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
    let seeds: Vec<u64> = lines
        .next()
        .expect("Can't read first line of file.")
        .expect("Can't read content of first line")
        .split_once(": ")
        .expect("Can't split on a colon.")
        .1
        .split(' ')
        .map(|s| s.parse::<u64>().expect("Can't parse seed num as u32."))
        .collect();

    let (mappings, map_len) = get_mappings(lines);

    let mut lowest_location = None;
    for &seed in seeds.iter() {
        let mut src_value = seed;
        let mut i = 0;
        while i < map_len {
            let current_maps = &mappings[i];
            for mapping in current_maps.iter() {
                let range = mapping.src..mapping.src + mapping.range;

                if range.contains(&src_value) {
                    src_value = src_value - mapping.src + mapping.dest;
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

fn get_seed_pairs(seed_line: &str) -> Vec<(u64, u64)> {
    let mut pairs: Vec<(u64, u64)> = Vec::new();
    let seeds: Vec<u64> = seed_line
        .split(": ")
        .last()
        .expect("Can't get last from split on \":\"!")
        .split(' ')
        .map(|s| s.parse::<u64>().expect("Can't parse seed to u64"))
        .collect();

    let sl = seeds.len();
    let mut i = 0;
    while i < sl {
        pairs.push((seeds[i], seeds[i] + seeds[i + 1]));
        i += 2;
    }

    return pairs;
}

fn find_mapping(value_range: (u64, u64), map_ranges: &[Mapping]) -> Vec<(u64, u64)> {
    let (vs, ve) = value_range;

    let mut found_ranges: Vec<(u64, u64)> = Vec::new();
    let mut found_it = false;

    for &Mapping { src, dest, range } in map_ranges {
        let overflow_start = max(vs, src);
        let overflow_end = min(ve, src + range);

        if overflow_start >= overflow_end {
            continue;
        }

        // check if our value range is fully contained within this mapping
        if vs < overflow_start {
            let mut map = find_mapping((vs, overflow_start), map_ranges);
            found_ranges.append(&mut map);
        }
        if ve > overflow_end {
            let mut map = find_mapping((overflow_end, ve), map_ranges);
            found_ranges.append(&mut map);
        }

        // we found a mapping
        found_it = true;
        let found_mapping = (overflow_start - src + dest, overflow_end - src + dest);
        found_ranges.push(found_mapping);
        break;
    }

    if !found_it {
        found_ranges.push(value_range);
    }

    return found_ranges;
}

// Definitely some hack n' slash here.
// Found it easiest to call out each section specifically and perform the same action
// n-1 times
pub fn part_two() {
    let mut lines = get_file_lines(INPUT_PATH);
    let first_line = lines
        .next()
        .expect("Can't get first line of file!")
        .expect("Can't get content from first line!");

    let (mappings, _) = get_mappings(lines);

    let seeds = get_seed_pairs(&first_line);
    let mut soils: Vec<(u64, u64)> = Vec::new();
    let soil_map = &mappings[0];
    for &seed in seeds.iter() {
        soils.append(&mut find_mapping(seed, soil_map));
    }

    let mut ferts: Vec<(u64, u64)> = Vec::new();
    let fert_map = &mappings[1];
    for &soil in soils.iter() {
        ferts.append(&mut find_mapping(soil, fert_map));
    }

    let mut waters: Vec<(u64, u64)> = Vec::new();
    let water_map = &mappings[2];
    for &fert in ferts.iter() {
        waters.append(&mut find_mapping(fert, water_map));
    }

    let mut lights: Vec<(u64, u64)> = Vec::new();
    let light_map = &mappings[3];
    for &water in waters.iter() {
        lights.append(&mut find_mapping(water, light_map));
    }

    let mut temps: Vec<(u64, u64)> = Vec::new();
    let temp_map = &mappings[4];
    for &light in lights.iter() {
        temps.append(&mut find_mapping(light, temp_map));
    }

    let mut humids: Vec<(u64, u64)> = Vec::new();
    let humid_map = &mappings[5];
    for &temp in temps.iter() {
        humids.append(&mut find_mapping(temp, humid_map));
    }

    let mut locations: Vec<(u64, u64)> = Vec::new();
    let location_map = &mappings[6];
    for &humid in humids.iter() {
        locations.append(&mut find_mapping(humid, location_map));
    }

    let answer = locations
        .iter()
        .min()
        .expect("Location iter().min() blew up")
        .0;
    assert_eq!(answer, 72263011);
    println!("Day 5 Part 2: {}", answer);
}
