//! Solution for Day 4 of the 2022 AOC (Advent Of Code)
use std::fs;

/// Finds how many pairs in `sets` completely overlap
fn find_overlap_sum(sets: &[&str]) -> u32 {
    let mut sum = 0;

    for s in sets {
        sum += s
            .split(|b: char| b == '-' || b == ',')
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .chunks(4)
            .map(|chunk| {
                if chunk[0] <= chunk[2] && chunk[3] <= chunk[1] {
                    1
                } else if chunk[2] <= chunk[0] && chunk[1] <= chunk[3] {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>();
    }

    return sum;
}

/// Finds how many pairs in `sets` overlap at all
fn find_touch_sum(sets: &[&str]) -> u32 {
    let mut sum = 0;

    for s in sets {
        sum += s
            .split(|b: char| b == '-' || b == ',')
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .chunks(4)
            .map(|chunk| {
                for n in chunk[0]..=chunk[1] {
                    if (chunk[2]..=chunk[3]).contains(&n) {
                        return 1;
                    }
                }
                return 0;
            })
            .sum::<u32>();
    }

    return sum;
}

fn main() {
    let file = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let lines = file.lines().collect::<Vec<&str>>();

    println!("> {}", find_overlap_sum(&lines));
    println!("> {}", find_touch_sum(&lines));
}
