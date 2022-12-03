use std::fs;

mod day_01;
mod day_02;
mod day_03;

fn main() {
    println!("Advent of Code 2022 Results:");

    // day 01: calorie counting part 1
    let day_one_input = fs::read_to_string("inputs/1.txt").unwrap();
    let day_one_result = day_01::part_one(&day_one_input);
    println!("day 01 part 1: {}", day_one_result);

    // day 01: calorie counting part 2
    let day_one_result = day_01::part_two(day_one_input);
    println!("day 01 part 2: {}", day_one_result);

    // day 02: rock paper scissors part 1
    let day_two_input = fs::read_to_string("inputs/2.txt").unwrap();
    let day_two_result = day_02::part_one(&day_two_input);
    println!("day 02 part 1: {}", day_two_result);

    // day 02: rock paper scissors part 2
    let day_two_result = day_02::part_two(day_two_input);
    println!("day 02 part 2: {}", day_two_result);

    // day 03: rucksack reorganization part 1
    let day_three_input = fs::read_to_string("inputs/3.txt").unwrap();
    let day_three_result = day_03::part_one(&day_three_input);
    println!("day 03 part 1: {}", day_three_result);

    // day 03: rucksack reorganization part 2
    let day_three_result = day_03::part_two(day_three_input);
    println!("day 03 part 2: {}", day_three_result);
}
