//! Solution for Day 11 of the 2022 AOC (Advent Of Code)
use std::fs;

#[derive(Clone, Debug, Default)]
struct Monkey {
    starting_items: Vec<u64>,
    operation: String,
    divisor: u64,
    if_true_monkey: usize,
    if_false_monkey: usize,
    inspected_items: usize
}

fn parse_monkey(lines: &[&str]) -> Monkey {
    let mut result = Monkey { ..Default::default() };

    for (pos, line) in lines.iter().enumerate() {
        match pos {
            1 => { // Starting items
                result.starting_items = line.replace("  Starting items: ", "")
                    .split(", ")
                    .map(|e| e.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
            },
            2 => { // Operation
                result.operation = line.replace("  Operation: new = ", "");
            },
            3 => { // Divisor
                result.divisor = line.replace("  Test: divisible by ", "").parse::<u64>().unwrap();
            },
            4 => { // If true branch
                result.if_true_monkey = line.replace("    If true: throw to monkey ", "")
                    .parse::<usize>().unwrap();
            },
            5 => { // If false branch
                result.if_false_monkey = line.replace("    If false: throw to monkey ", "")
                    .parse::<usize>().unwrap();
            },
            _ => {}
        }
    }

    result
}

fn do_round(monkeys: &mut Vec<Monkey>, div_three: bool) -> Vec<(usize, u64)> {
    let mut moved: Vec<(usize, u64)> = vec![];
    
    let apply_operation = |op: &String, num: u64| {
        let data = op.replace("old", &num.to_string());
        let (pre, expr) = data.trim().split_once(" ").unwrap();
        let (ex, post) = expr.split_once(" ").unwrap();

        match ex {
            "+" => (pre.parse::<u64>().unwrap() + post.parse::<u64>().unwrap()),
            "-" => (pre.parse::<u64>().unwrap() - post.parse::<u64>().unwrap()),
            "*" => (pre.parse::<u64>().unwrap() * post.parse::<u64>().unwrap()),
            "/" => (pre.parse::<u64>().unwrap() / post.parse::<u64>().unwrap()),
            _ => (0)
        }
    };

    for (pos, m) in monkeys.into_iter().enumerate() {
        moved.iter().filter(|e| e.0 == pos).for_each(|e| m.starting_items.push(e.1));
        moved.retain(|e| e.0 != pos);

        for item in m.starting_items.drain(..) {
            let new_item: u64 = if div_three {
                apply_operation(&m.operation, item) / 3
            } else {
                apply_operation(&m.operation, item % 9699690)
            };

            if new_item % m.divisor == 0 {
                moved.push((m.if_true_monkey, new_item));
            } else {
                moved.push((m.if_false_monkey, new_item));
            }

            m.inspected_items += 1;
        }
    }

    moved
}

fn main() {
    let data = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut monkeys = data.lines()
                          .collect::<Vec<&str>>()
                          .chunks(7)
                          .map(|g| parse_monkey(g))
                          .collect::<Vec<Monkey>>();
    let mut monkeys2 = monkeys.clone();
    
    let get_monkey_buisness = |set: &mut Vec<Monkey>| {
        set.sort_by(|a, b| a.inspected_items.partial_cmp(&b.inspected_items).unwrap());
        set[set.len() - 1].inspected_items * set[set.len() - 2].inspected_items
    };
    
    for _ in 0..10000 {
        do_round(&mut monkeys, false).iter().for_each(|(i, e)| monkeys[*i].starting_items.push(*e));
    }
    
    for _ in 0..20 {
        do_round(&mut monkeys2, true).iter().for_each(|(i, e)| monkeys2[*i].starting_items.push(*e));
    }

    println!("> {}", get_monkey_buisness(&mut monkeys2));
    println!("> {}", get_monkey_buisness(&mut monkeys));
}
