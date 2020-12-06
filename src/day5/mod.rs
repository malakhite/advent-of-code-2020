use std::fs::{canonicalize};
use std::{error};

#[derive(Debug)]
enum RowOrCol {
    Row,
    Col,
}

fn calc_partial_id(s: &str, roc: RowOrCol) -> u64 {
    let mut id: u64 = 0;
    let mut min: u64 = 0;
    let mut max: u64 = match roc {
        RowOrCol::Row => 127,
        RowOrCol::Col => 7,
    };
    for c in s.chars() {
        // println!("max: {}, min: {}", max, min);
        if max - min == 1 {
            if c == 'F' || c == 'L' {
                id = min;
            } else if c == 'B' || c == 'R' {
                id = max;
            } else {
                panic!("Unable to parse character: {}", c);
            }
        } else if c == 'F' || c == 'L' {
            max = max - ((max - min) / 2 + 1);
        } else if c == 'B' || c == 'R' {
            min = min + ((max - min) / 2 + 1);
        } else {
            panic!("Unable to parse character: {}", c);
        }
    }

    // println!("The {:#?} id is {}", roc, id);

    id
}

fn calc_id(s: &String) -> (u64, u64, u64) {
    let (row, col) = s.split_at(7);
    let row_id = calc_partial_id(row, RowOrCol::Row);
    let col_id = calc_partial_id(col, RowOrCol::Col);

    let seat_id = row_id * 8 + col_id;
    // println!("The seat id is {}", seat_id);
    (seat_id, row_id, col_id)
}

pub fn part1() -> Result<(), Box<dyn error::Error>> {
    let path = canonicalize("./puzzle-input/day5.txt")?;
    let contents = super::util::read_lines(path)?;
    let mut max: u64 = 0;
    for line in contents {
        let s = String::from(line.unwrap());
        let (id, _, _) = calc_id(&s);
        if id > max {
            max = id;
        }
    }
    println!("The max seat id is: {}", max);

    Ok(())
}

pub fn part2() -> Result<(), Box<dyn error::Error>> {
    let path = canonicalize("./puzzle-input/day5.txt")?;
    let contents = super::util::read_lines(path)?;
    let mut ids: Vec<u64> = Vec::new();
    for line in contents {
        let s = String::from(line.unwrap());
        let (seat_id, _, _) = calc_id(&s);
        ids.push(seat_id);
    }
    ids.sort();
    for (index, id) in ids.iter().enumerate() {
        if index > 0 && index < ids.len() - 2 {
            if id + 1 != ids[index + 1] {
                println!("ID {} is missing", id + 1);
            }
        }
    }

    Ok(())
}