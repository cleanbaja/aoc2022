//! Solution for Day 9 of the 2022 AOC (Advent Of Code)
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Hash, Copy, Clone, Eq, PartialEq)]
struct Point2D<T>(T, T);

impl<T: std::ops::Add<Output = T>> std::ops::Add for Point2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Point2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn find_tail_visited(data: &[(&str, i16)], movs: &HashMap<&str, Point2D<i16>>) -> usize {
    let mut tail_visited = HashSet::new();
    let mut head = Point2D(0i16, 0i16);
    let mut tail = Point2D(0i16, 0i16);
    tail_visited.insert(tail);

    for mov in data {
        for _ in 0..=mov.1 - 1 {
            head = head + movs[mov.0];
            let diff = head - tail;

            if diff.0.abs() > 1 || diff.1.abs() > 1 {
                tail.0 += diff.0.signum();
                tail.1 += diff.1.signum();
                tail_visited.insert(tail);
            }
        }
    }

    tail_visited.len()
}

fn find_rope_visited(data: &[(&str, i16)], movs: &HashMap<&str, Point2D<i16>>) -> usize {
    let mut tail_visited = HashSet::new();
    let mut pieces: [Point2D<i16>; 10] = [Point2D(0, 0); 10];
    tail_visited.insert(Point2D(0i16, 0i16));

    for mov in data {
        for _ in 0..mov.1 {
            pieces[0] = pieces[0] + movs[mov.0];

            for n in 0..9 {
                let diff = pieces[n] - pieces[n + 1];

                if diff.0.abs() > 1 || diff.1.abs() > 1 {
                    pieces[n + 1].0 += diff.0.signum();
                    pieces[n + 1].1 += diff.1.signum();

                    if n == 8 {
                        tail_visited.insert(pieces[n + 1]);
                    }
                }
            }
        }
    }

    tail_visited.len()
}

fn main() {
    let data = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let instructions = data
        .lines()
        .map(|l| {
            // turn the lines into tuples
            let (instr, off) = l.split_once(" ").unwrap();
            (instr, off.parse::<i16>().unwrap())
        })
        .collect::<Vec<(&str, i16)>>();

    let movs: HashMap<&str, Point2D<i16>> = HashMap::from([
        ("U", Point2D(0, 1)),
        ("D", Point2D(0, -1)),
        ("L", Point2D(-1, 0)),
        ("R", Point2D(1, 0)),
    ]);

    println!("> {}", find_tail_visited(&instructions, &movs));
    println!("> {}", find_rope_visited(&instructions, &movs));
}
