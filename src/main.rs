mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    match day1::part1() {
        Ok(_) => println!("Day 1 part 1 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day1::part2() {
        Ok(_) => println!("Day 1 part 2 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day2::part1() {
        Ok(_) => println!("Day 2 part 1 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day2::part2() {
        Ok(_) => println!("Day 2 part 2 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day3::part1() {
        Ok(_) => println!("Day 3 part 1 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day3::part2() {
        Ok(_) => println!("Day 3 part 2 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day4::part1() {
        Ok(_) => println!("Day 4 part 1 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day4::part2() {
        Ok(_) => println!("Day 4 part 2 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day5::part1() {
        Ok(_) => println!("Day 5 part 1 success"),
        Err(e) => println!("Error: {}", e),
    }

    match day5::part2() {
        Ok(_) => println!("Day 5 part 2 success"),
        Err(e) => println!("Error: {}", e),
    }

    day6::part1();

    day6::part2();
}
