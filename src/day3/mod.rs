use std::{error};
use std::fs::{canonicalize};

pub fn part1() -> Result<(), Box<dyn error::Error>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let path = canonicalize("./puzzle-input/day3.1.txt")?;
    let contents = super::util::read_lines(path)?;
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
    let contents = super::util::read_lines(path)?;
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

pub fn part2() -> Result<(), Box<dyn error::Error>> {
    let a = count_trees(1, 1)?;
    let b = count_trees(3, 1)?;
    let c = count_trees(5, 1)?;
    let d = count_trees(7, 1)?;
    let e = count_trees(1, 2)?;
    let total: u64 = a * b * c * d * e;
    println!("The product of the tree counts is {}", total);

    Ok(())
}