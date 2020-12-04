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

fn day3_part1() -> Result<(), Box<dyn error::Error>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let path = canonicalize("./puzzle-input/day3.1.txt")?;
    let contents = read_lines(path)?;
    for line in contents {
        let v: Vec<char> = String::from(line.unwrap()).chars().collect();
        grid.push(v);
    }

    let mut trees: u32 = 0;
    let mut x: usize = 0;
    for row in grid.iter() {
        if row[x] == '#' {
            trees += 1;
        }
        x = (x + 3) % row.len();
    }

    println!("The number of trees encountered is: {}", trees);
    Ok(())
}

fn count_trees(across: usize, down: usize) -> Result<u64, Box<dyn error::Error>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let path = canonicalize("./puzzle-input/day3.1.txt")?;
    let contents = read_lines(path)?;
    for line in contents {
        let v: Vec<char> = String::from(line.unwrap()).chars().collect();
        grid.push(v);
    }

    let mut trees: u64 = 0;
    let mut x: usize = 0;
    for (index, row) in grid.iter().enumerate() {
        if index % down == 0 {
            if row[x] == '#' {
                trees += 1;
            }
            x = (x + across) % row.len();
        }
    }

    println!("For a slope of {} across and {} down, the number of trees encountered is: {}", across, down, trees);
    Ok(trees)
}

fn day3_part2() -> Result<(), Box<dyn error::Error>> {
    let a = count_trees(1, 1)?;
    let b = count_trees(3, 1)?;
    let c = count_trees(5, 1)?;
    let d = count_trees(7, 1)?;
    let e = count_trees(1, 2)?;
    let total: u64 = a * b * c * d * e;
    println!("The product of the tree counts is {}", total);

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
