//! Solution for Day 3 of the 2022 AOC (Advent Of Code)
use std::fs;

/// Returns the priority for `ch` as described in the instructions
#[inline]
fn find_priority(ch: char) -> u32 {
    if ch.is_uppercase() {
        return ch as u32 - 38;
    } else {
        return ch as u32 - 96;
    }
}

/// Takes a slice of [&str], and finds the priority of 
/// each elem (rucksack), returning the sum
fn get_prio_sum(lines: &[&str]) -> u32 {
    let mut score = 0;

    for line in lines {
        let mut chars: Vec<char> = line.chars().collect();
        let chars2 = chars.split_off(chars.len() / 2);

        chars.retain(|c| chars2.contains(c));
        score += find_priority(chars[0]);
    }

    return score;
}

/// Finds the priority of every 3 elements in `lines`, and returns the sum
fn get_badge_sum(lines: &[&str]) -> u32 {
    let mut score = 0;

    'set: for set in lines.chunks(3) {
        for ch in set[0].chars() {
            if set[1].contains(ch) && set[2].contains(ch) {
                score += find_priority(ch);
                continue 'set;
            }
        }
    }

    return score;
}

fn main() {
    let file = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let lines = file.lines().collect::<Vec<&str>>();

    println!("> {}", get_prio_sum(&lines));
    println!("> {}", get_badge_sum(&lines));
}
