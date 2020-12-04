use std::{error};
use std::fs::{canonicalize};

pub fn day1_part1() -> Result<(), Box<dyn error::Error>> {
    const TARGET: i64 = 2020;
    let mut v: Vec<i64> = Vec::new();
    let path = canonicalize("./puzzle-input/day1.1.txt")?;
    let contents = super::util::read_lines(path)?;
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

pub fn day1_part2() -> Result<(), Box<dyn error::Error>> {
    const TARGET: i64 = 2020;
    let mut v: Vec<i64> = Vec::new();
    let path = canonicalize("./puzzle-input/day1.1.txt")?;
    let contents = super::util::read_lines(path)?;
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