//! Solution for Day 2 of the 2022 AOC (Advent Of Code)
use std::fs;

/// Parses each line in `lines`, according to the instructions
fn get_score(lines: &Vec<&str>) -> u32 {
    let mut score = 0;

    for line in lines {
        match *line {
            // rock + rock -> draw
            "A X" => score += 4,
            // rock + paper -> win
            "A Y" => score += 8,
            // rock + scissors -> loss
            "A Z" => score += 3,

            // paper + rock -> loss
            "B X" => score += 1,
            // paper + paper -> draw
            "B Y" => score += 5,
            // paper + scissors -> win
            "B Z" => score += 9,
            
            // scissors + rock -> win
            "C X" => score += 7,
            // scissors + paper -> loss
            "C Y" => score += 2,
            // scissors + scissors -> draw
            "C Z" => score += 6,

            _ => eprintln!("unknown input {}!", *line)
        }
    }

    return score;
}

/// Parses each line in `lines`, according to the instructions
fn get_score_type2(lines: &Vec<&str>) -> u32 {
    let mut score = 0;

    for line in lines {
        match *line {
            // rock + lose -> scissors
            "A X" => score += 3,
            // rock + draw -> rock
            "A Y" => score += 4,
            // rock + win -> paper
            "A Z" => score += 8,

            // paper + lose -> rock
            "B X" => score += 1,
            // paper + draw -> paper
            "B Y" => score += 5,
            // paper + win -> scissors
            "B Z" => score += 9,
            
            // scissors + lose -> paper
            "C X" => score += 2,
            // scissors + draw -> scissors
            "C Y" => score += 6,
            // scissors + win -> rock
            "C Z" => score += 7,

            _ => eprintln!("unknown input {}!", *line)
        }
    }

    return score;
}

fn main() {
    let file = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let lines = file.lines().collect::<Vec<&str>>();

    println!("> {}", get_score(&lines));
    println!("> {}", get_score_type2(&lines));
}
