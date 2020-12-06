use std::{error};
use std::fs::{canonicalize};

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

pub fn part1() -> Result<(), Box<dyn error::Error>> {
    let mut count: u32 = 0;
    let path = canonicalize("./puzzle-input/day2.1.txt")?;
    let contents = super::util::read_lines(path)?;
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

pub fn part2() -> Result<(), Box<dyn error::Error>> {
    let mut count: u32 = 0;
    let path = canonicalize("./puzzle-input/day2.1.txt")?;
    let contents = super::util::read_lines(path)?;
    for line in contents {
        let s = String::from(line.unwrap());
        if parse_lines2(s)? {
            count += 1;
        }
    }

    println!("The number of valid passwords: {}", count);

    Ok(())
}