//! Solution for Day 7 of the 2022 AOC (Advent Of Code)
use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut dirs: HashMap<String, usize> = HashMap::new();
    let mut cur_dirs = vec!["/".to_string()];

    let output = data.lines().filter(|e| *e != "$ ls").skip(1);
    dirs.insert("/".to_string(), 0);

    for elem in output {
        if elem == "$ cd .." {
            cur_dirs.remove(cur_dirs.len() - 1);
        } else if elem.contains("$ cd ") {
            cur_dirs.push(elem.replace("$ cd ", ""));
        } else {
            let (prefix, filename) = elem.split_once(" ").unwrap();

            if prefix == "dir" {
                dirs.insert(cur_dirs.join("/") + "/" + filename, 0);
            } else {
                for (pos, dir) in cur_dirs.iter().enumerate() {
                    let filename = if dir == "/" {
                        dir.to_string()
                    } else {
                        cur_dirs[..pos].join("/") + "/" + dir
                    };

                    *dirs.get_mut(&filename).unwrap() += prefix.parse::<usize>().unwrap();
                }
            }
        }
    }

    let target_size = 30000000 - (70000000 - dirs["/"]);
    let mut values = dirs.into_values().collect::<Vec<usize>>();
    values.sort();

    println!(
        "> {}",
        values.iter().filter(|e| **e <= 100000).sum::<usize>()
    );
    println!("> {}", values.iter().find(|e| **e > target_size).unwrap());
}
