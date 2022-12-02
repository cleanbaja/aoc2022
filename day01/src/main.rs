//! Solution for Day 1 of the 2022 AOC (Advent Of Code)
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/// Opens `filename` and parses it into a vector of strings
fn read_lines(filename: String) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in buf.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

/// Generates a vector of [u32]s, representing the calorie count
/// for each elf.
fn generate_set(lines: &Vec<String>) -> io::Result<Vec<u32>> {
    let mut dataset: Vec<u32> = Vec::new();
    let mut counter = 0;

    for line in lines {
        if line.trim().is_empty() {
            dataset.push(counter);
            counter = 0;
        } else {
            counter += line.trim().parse::<u32>().unwrap();
        }
    }

    Ok(dataset)
}


/// Generates a set from `lines`, then returns the largest
/// number in the set
#[inline]
fn compute_sum(lines: &Vec<String>) -> u32 {
    *generate_set(lines).unwrap().iter().max().unwrap()
}

/// Generates a set from `lines` sorting it from largest to smallest.
/// It then returns the sum of the first three elements in the set.
fn compute_top_three(lines: &Vec<String>) -> u32 {
    let mut dataset = generate_set(lines).unwrap();
    dataset.sort();
    dataset.reverse();

    return dataset[0] + dataset[1] + dataset[2];
}

fn main() {
    let lines = read_lines(std::env::args().nth(1).unwrap()).unwrap();
    println!("> {}", compute_sum(&lines));
    println!("> {}", compute_top_three(&lines));
}
