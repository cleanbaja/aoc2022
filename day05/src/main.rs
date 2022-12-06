//! Solution for Day 5 of the 2022 AOC (Advent Of Code)
use std::fs;

/// Parses `lines` for the initial stack layout, and
/// returns it in a structure fit for consumption
fn get_stacks(lines: &[&str]) -> [Vec<char>; 9] {
    let mut stacks: [Vec<char>; 9] = [
        Vec::new(), Vec::new(), Vec::new(),
        Vec::new(), Vec::new(), Vec::new(),
        Vec::new(), Vec::new(), Vec::new(),
    ];

    for num in 0..9 {
        for line in lines {
            let ch = line.chars().nth(num * 4 + 1).unwrap();
            if ch != ' ' {
                stacks[num].push(ch);
            }
        }

        stacks[num].reverse();
    }

    return stacks;
}

/// parses `lines` for operations to perform on `sets`, before returning
/// the topmost element for every stack in `sets`
fn do_crate_moves(lines: &[&str], sets: &[Vec<char>; 9], reverse: bool) -> String {
    let mut stacks: [Vec<char>; 9] = [
        Vec::new(), Vec::new(), Vec::new(),
        Vec::new(), Vec::new(), Vec::new(),
        Vec::new(), Vec::new(), Vec::new(),
    ];
    stacks.clone_from_slice(sets);

    let numbers = lines
        .iter()
        .map(|s| s.replace("move", "").replace("from", "").replace("to", ""))
        .map(|s| {
            let (_, rem) = s.split_once(" ").unwrap();
            let (one, rem) = rem.split_once(" ").unwrap();
            let (_, rem) = rem.split_once(" ").unwrap();
            let (two, three) = rem.split_once(" ").unwrap();

            (
                one.parse::<u16>().unwrap(),
                two.parse::<u16>().unwrap(),
                three.trim().parse::<u16>().unwrap(),
            )
        })
        .collect::<Vec<(u16, u16, u16)>>();

    for set in numbers {
        let (num_crates, src, dest) = set;
        let mut old_elem: Vec<char> = stacks[src as usize - 1]
            .drain((stacks[src as usize - 1].len() - num_crates as usize)..)
            .collect::<Vec<char>>();

        if reverse {
            old_elem.reverse();
        }

        stacks[dest as usize - 1].extend(old_elem);
    }

    format!(
        "{}{}{}{}{}{}{}{}{}",
        stacks[0][stacks[0].len() - 1],
        stacks[1][stacks[1].len() - 1],
        stacks[2][stacks[2].len() - 1],
        stacks[3][stacks[3].len() - 1],
        stacks[4][stacks[4].len() - 1],
        stacks[5][stacks[5].len() - 1],
        stacks[6][stacks[6].len() - 1],
        stacks[7][stacks[7].len() - 1],
        stacks[8][stacks[8].len() - 1]
    )
}

fn main() {
    let file = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let lines = file.lines().collect::<Vec<&str>>();

    let stacks = get_stacks(&lines[..8]);
    println!("> {}", do_crate_moves(&lines[10..], &stacks, true));
    println!("> {}", do_crate_moves(&lines[10..], &stacks, false));
}
