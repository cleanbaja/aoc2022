//! Solution for Day 12 of the 2022 AOC (Advent Of Code)
use std::{
    collections::{BinaryHeap, HashSet},
    fs::read_to_string,
};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Coord(usize, usize);

#[derive(Debug, Eq, PartialEq)]
struct Node {
    score: u32,
    pos: Coord,
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Node) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_start_end(set: &mut Vec<Vec<char>>) -> (Coord, Coord) {
    let mut start = Coord(0, 0);
    let mut end = Coord(0, 0);

    for (y, row) in set.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if *elem == 'S' {
                start = Coord(x, y);
            } else if *elem == 'E' {
                end = Coord(x, y);
            }
        }
    }

    set[start.1][start.0] = 'a';
    set[end.1][end.0] = 'z';

    (start, end)
}

fn get_neighbors(elem: Coord, rows: usize, cols: usize) -> Vec<Coord> {
    let mut result = Vec::new();

    if elem.1 > 0 {
        result.push(Coord(elem.0, elem.1 - 1));
    }

    if elem.1 < rows - 1 {
        result.push(Coord(elem.0, elem.1 + 1));
    }

    if elem.0 > 0 {
        result.push(Coord(elem.0 - 1, elem.1));
    }

    if elem.0 < cols - 1 {
        result.push(Coord(elem.0 + 1, elem.1));
    }

    result
}

fn solve_path(set: &Vec<Vec<char>>, start: Coord, end: Coord, part2: bool) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::from([start]);
    let rows = set.len();
    let cols = set[0].len();

    queue.push(Node {
        score: 0,
        pos: if part2 { end } else { start },
    });

    while let Some(Node { score, pos }) = queue.pop() {
        if part2 {
            if set[pos.1][pos.0] == 'a' {
                return score;
            }
        } else {
            if pos == end {
                return score;
            }
        }

        let peers = get_neighbors(pos, rows, cols);
        let candidates = peers
            .into_iter()
            .filter(|p| {
                let new_elem = set[p.1][p.0] as u32;
                if part2 {
                    new_elem >= set[pos.1][pos.0] as u32 || new_elem == set[pos.1][pos.0] as u32 - 1
                } else {
                    new_elem <= set[pos.1][pos.0] as u32 || new_elem == set[pos.1][pos.0] as u32 + 1
                }
            })
            .collect::<Vec<Coord>>();

        for c in candidates {
            if seen.insert(c) {
                queue.push(Node {
                    score: score + 1,
                    pos: c,
                })
            }
        }
    }

    0
}

fn main() {
    let data = read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut grid = data
        .lines()
        .map(|e| e.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let (start, end) = find_start_end(&mut grid);

    println!("> {}", solve_path(&grid, start, end, false));
    println!("> {}", solve_path(&grid, start, end, true));
}
