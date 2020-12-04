mod util;

mod day1;
use day1::{day1_part1, day1_part2};

mod day2;
use day2::{day2_part1, day2_part2};

mod day3;
use day3::{day3_part1, day3_part2};

fn main() {
    match day1_part1() {
        Ok(_) => println!("Day 1 part 1 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day1_part2() {
        Ok(_) => println!("Day 1 part 2 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day2_part1() {
        Ok(_) => println!("Day 2 part 1 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day2_part2() {
        Ok(_) => println!("Day 2 part 2 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day3_part1() {
        Ok(_) => println!("Day 3 part 1 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day3_part2() {
        Ok(_) => println!("Day 3 part 1 success"),
        Err(e) => println!("Error: {}", e),
    }
}
