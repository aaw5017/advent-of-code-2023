#[macro_use]
extern crate lazy_static;

mod day_five;
mod day_four;
mod day_one;
mod day_three;
mod day_two;

fn main() {
    // Day One
    day_one::part_one();
    day_one::part_two();

    // Day Two
    day_two::part_one();
    day_two::part_two();

    // Day Three
    day_three::part_one();
    day_three::part_two();

    // Day Four
    day_four::part_one();
    day_four::part_two();

    // Day Five
    day_five::part_one();
    day_five::part_two();
}
