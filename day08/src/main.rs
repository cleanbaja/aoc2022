//! Solution for Day 8 of the 2022 AOC (Advent Of Code)
use std::fs;

fn transpose<T: Clone>(vec: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..vec[0].len())
        .map(|i| vec.iter().map(|j| j[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn check_vis<T>(set: &Vec<Vec<T>>, x: usize, y: usize) -> bool
where
    T: std::cmp::PartialOrd,
{
    let mut top_sum = 0;
    let mut bottom_sum = 0;
    let mut left_sum = 0;
    let mut right_sum = 0;
    let elem = &set[y][x];

    for (pos, row) in set.iter().enumerate() {
        if pos < y {
            // top sum
            if row[x] < *elem {
                top_sum += 1;
            }
        } else if pos > y {
            // bottom sum
            if row[x] < *elem {
                bottom_sum += 1;
            }
        }
    }

    for (pos, col) in set[y].iter().enumerate() {
        if pos > x {
            // right sum
            if *col < *elem {
                right_sum += 1;
            }
        } else if pos < x {
            // left sum
            if *col < *elem {
                left_sum += 1;
            }
        }
    }

    if top_sum >= y
        || bottom_sum >= (set.len() - 1 - y)
        || left_sum >= x
        || right_sum >= set[y].len() - 1 - x
    {
        return true;
    }

    return false;
}

fn get_score(set: &Vec<Vec<i8>>, rset: &Vec<Vec<i8>>, x: usize, y: usize) -> usize {
    let elem = &set[y][x];
    let left = &set[y][x + 1..];
    let bottom = &rset[x][y + 1..];
    let right = &set[y][..x].iter().rev().map(|e| *e).collect::<Vec<i8>>();
    let top = &rset[x][..y].iter().rev().map(|e| *e).collect::<Vec<i8>>();

    let do_score = |tree, elems: &[i8]| {
        let mut cnt = 0;

        for e in elems {
            cnt += 1;

            if e >= tree {
                break;
            }
        }

        cnt
    };

    (do_score(elem, &top)
        * do_score(elem, &bottom)
        * do_score(elem, &left)
        * do_score(elem, &right)) as usize
}

fn num_visible_trees(readings: &Vec<Vec<i8>>) -> usize {
    let mut visible = 2 * (readings.len() + readings[0].len()) - 4;
    for y in 1..=readings.len() - 2 {
        for x in 1..readings[0].len() - 1 {
            if check_vis(&readings, x, y) {
                visible += 1;
            }
        }
    }

    return visible;
}

fn find_top_score(readings: &Vec<Vec<i8>>) -> usize {
    let mut scores = Vec::new();
    let flipped = transpose(readings);

    for y in 0..=readings.len() - 1 {
        for x in 0..readings[0].len() {
            scores.push(get_score(&readings, &flipped, x, y));
        }
    }

    *scores.iter().max().unwrap()
}

fn main() {
    let data = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let treemap = data
        .lines()
        .map(|e| {
            e.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();

    println!("> {}", num_visible_trees(&treemap));
    println!("> {}", find_top_score(&treemap));
}
