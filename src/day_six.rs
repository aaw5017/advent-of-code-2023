use crate::util::get_file_lines;

const INPUT_PATH: &'static str = "data/day_six/puzzle_input.txt";
// const INPUT_PATH: &'static str = "data/day_six/test_input.txt";

#[derive(Debug)]
struct Race {
    time_ms: u16,
    record_distance: u16,
}
impl Race {
    fn new(time_ms: u16, record_distance: u16) -> Self {
        return Race {
            time_ms,
            record_distance,
        };
    }
}

fn get_races() -> Vec<Race> {
    let mut lines = get_file_lines(INPUT_PATH);
    let mut races: Vec<Race> = Vec::new();

    let time_line = lines
        .next()
        .expect("Can't get Time: line")
        .expect("Can't get Time: content");

    let times = time_line
        .split_once(':')
        .expect("Can't split on :")
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u16>().expect("Cannot parse time u16"))
        .collect::<Vec<_>>();

    let distance_line = lines
        .next()
        .expect("Can't get Distance: eine")
        .expect("Can't get Distance: content");

    let distances = distance_line
        .split_once(':')
        .expect("Can't split on :")
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u16>().expect("Cannot parse time u16"))
        .collect::<Vec<_>>();

    for (i, &t) in times.iter().enumerate() {
        races.push(Race::new(t, distances[i]));
    }

    return races;
}

pub fn part_one() {
    let races = get_races();

    let mut moe = 1;
    for race in races.iter() {
        let mut win_count = 0;
        for time_held in 1..race.time_ms {
            let my_dist = time_held * (race.time_ms - time_held);
            if my_dist > race.record_distance {
                win_count += 1;
            }
        }

        if win_count > 0 {
            moe *= win_count;
        }
    }

    assert_eq!(moe, 219849);
    println!("Day 6 Part 1: {}", moe);
}

pub fn part_two() {}
