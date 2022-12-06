//! Solution for Day 6 of the 2022 AOC (Advent Of Code)
use std::cmp::Ord;
use std::fs;

// Checks if `slice` contains duplicate elements
fn check_duplicates<T: Eq + Copy + Ord>(slice: &[T]) -> bool {
    let mut data = slice.to_vec();
    data.sort();
    data.dedup();

    data.len() != slice.len()
}

// finds the marker in `file`, which is `step` consecutive unique numbers
fn find_marker(file: &str, step: u16) -> u16 {
    file.chars()
        .collect::<Vec<char>>()
        .windows(step as usize)
        .try_fold(0, |acc, e| {
            if check_duplicates(&e) {
                return Ok(acc + 1);
            } else {
                return Err(acc + step);
            }
        })
        .unwrap_err()
}

fn main() {
    let file = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("> {}", find_marker(&file, 4));
    println!("> {}", find_marker(&file, 14));
}
