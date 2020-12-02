use std::{error};
use std::fs::{canonicalize, File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day1_part1() -> Result<(), Box<dyn error::Error>> {
    const TARGET: i64 = 2020;
    let mut v: Vec<i64> = Vec::new();
    let path = canonicalize("./puzzle-input/day1.1.txt")?;
    let contents = read_lines(path)?;
    for line in contents {
        let i = String::from(line.unwrap()).parse::<i64>()?;
        v.push(i);
    }

    for (i, x) in v.iter().enumerate() {
        for y in v.iter().skip(i + 1) {
            if x + y == TARGET {
                let solution = x * y;
                println!("Found values {} and {} that sum to {}", x, y, TARGET);
                println!("The product of {} and {} is {}", x, y, solution);
                break;
            }
        }
    }

    Ok(())
}

fn day1_part2() -> Result<(), Box<dyn error::Error>> {
    const TARGET: i64 = 2020;
    let mut v: Vec<i64> = Vec::new();
    let path = canonicalize("./puzzle-input/day1.1.txt")?;
    let contents = read_lines(path)?;
    for line in contents {
        let i = String::from(line.unwrap()).parse::<i64>()?;
        v.push(i);
    }

    for (i, x) in v.iter().enumerate() {
        for (j, y) in v.iter().enumerate().skip(i + 1) {
            for z in v.iter().skip(j + 1) {
                if x + y + z == TARGET {
                    let solution = x * y * z;
                    println!("Found values {}, {}, and {} that sum to {}", x, y, z, TARGET);
                    println!("The product of {}, {}, and {} is {}", x, y, z, solution);
                    break;
                }
            }
        }
    }

    Ok(())
}


fn parse_lines(s: String) -> Result<bool, Box<dyn error::Error>> {
    let parts: Vec<&str> = s.split(" ").collect();
    let first: Vec<&str> = parts[0].clone().split("-").collect();
    let (min, max) = (first[0].parse::<usize>().unwrap(), first[1].parse::<usize>().unwrap());
    let token = parts[1].trim_end_matches(":");
    let matches: Vec<&str> = parts[2].matches(token).collect();
    if matches.len() <= max && matches.len() >= min {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn day2_part1() -> Result<(), Box<dyn error::Error>> {
    let mut count: u32 = 0;
    let path = canonicalize("./puzzle-input/day2.1.txt")?;
    let contents = read_lines(path)?;
    for line in contents {
        let s = String::from(line.unwrap());
        if parse_lines(s)? {
            count += 1;
        }
    }

    println!("The number of valid passwords: {}", count);

    Ok(())
}

fn parse_lines2(s: String) -> Result<bool, Box<dyn error::Error>> {
    let parts: Vec<&str> = s.split(" ").collect();
    let first: Vec<&str> = parts[0].clone().split("-").collect();
    let (a, b) = (first[0].parse::<usize>().unwrap() - 1, first[1].parse::<usize>().unwrap() - 1);
    let token = parts[1].chars().collect::<Vec<char>>()[0];
    let password: Vec<char> = parts[2].chars().collect();
    if token == password[a] && token == password[b] {
        Ok(false)
    } else if token == password[a] || token == password[b] {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn day2_part2() -> Result<(), Box<dyn error::Error>> {
    let mut count: u32 = 0;
    let path = canonicalize("./puzzle-input/day2.1.txt")?;
    let contents = read_lines(path)?;
    for line in contents {
        let s = String::from(line.unwrap());
        if parse_lines2(s)? {
            count += 1;
        }
    }

    println!("The number of valid passwords: {}", count);

    Ok(())
}

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
        Ok(_) => println!("Day 2 part 1 success"),
        Err(e) => println!("Error: {}", e),
    }
}
