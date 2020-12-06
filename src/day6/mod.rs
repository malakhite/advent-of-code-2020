use super::util::read_file;
use std::collections::{HashSet};

pub fn part1() {
    let file = String::from("./puzzle-input/day6.txt");
    let input = match read_file(&file) {
        Ok(inp) => inp,
        Err(why) => panic!("Unable to read file {}: {}", file, why),
    };
    let groups: Vec<&str> = input.split("\n\n").collect();
    let mut sum: usize = 0;
    for group in groups {
        let split_group: Vec<&str> = group.split("\n").collect();
        let mut yesses: HashSet<char> = HashSet::new();
        for person in split_group {
            for c in person.chars() {
                if c.is_alphabetic() {
                    yesses.insert(c);
                }
            }
        }
        sum = sum + yesses.len();
    }
    println!("Day 6 Part 1 is: {}", sum);
}

// Blatantly stolen from https://github.com/Lakret/aoc2020/blob/master/src/d6.rs#L3-L39
pub fn part2() {
    let file = String::from("./puzzle-input/day6.txt");
    let input = match read_file(&file) {
        Ok(inp) => inp,
        Err(why) => panic!("Unable to read file {}: {}", file, why),
    };
    let sum_of_counts: i64 = input.trim_end().split("\n\n")
        .map(|group| {
            let answers_per_person = group
                .split_ascii_whitespace()
                .map(|person| person.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();
            
            answers_per_person.iter()
                .fold(answers_per_person[0].clone(), |all_yes, persons_answers| {
                    all_yes.intersection(persons_answers).cloned().collect()
                })
                .len() as i64
        }).sum();
    println!("Day 6 Part 2 is: {}", sum_of_counts);
}