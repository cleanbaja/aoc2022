//! Solution for Day 10 of the 2022 AOC (Advent Of Code)
use std::fs;

#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop
}

/// finds the "xreg_sum" of `instrs` at certain clock cycles
fn find_xreg_sums(instrs: &[Instruction]) -> u32 {
    let mut xreg = 1;
    let mut cycles = 0;
    let mut result = 0;

    let mut check_cycles = |c, x| {
        if (c - 20) % 40 == 0 {
            result += x * c;
        }
    };

    for inst in instrs {
        cycles += 1;
        check_cycles(cycles, xreg);
    
        match inst {
            Instruction::Noop => { continue; },
            Instruction::AddX(imm) => {
                cycles += 1;
                check_cycles(cycles, xreg);

                xreg += imm;
            }
        }
    }

    result as u32
}


/// executes `instrs`, drawing a pixel every iteration to the grid
fn draw_crt(instrs: &[Instruction]) -> [[char; 40]; 6] {
    let mut grid: [[char; 40]; 6] = [['.'; 40]; 6];
    let mut cycles = 0i32;
    let mut xreg = 1i32;

    let mut draw_pixel = |x| {
        let margin = cycles % 40;
        
        let ch = if x == margin || x == (margin - 1) || x == (margin + 1) {
            '#'
        } else {
            '.'
        };
        
        grid[cycles as usize / 40][cycles as usize % 40] = ch;
        cycles += 1;
    };

    for inst in instrs {
        draw_pixel(xreg);
    
        match inst {
            Instruction::Noop => { continue; },
            Instruction::AddX(imm) => {
                draw_pixel(xreg);
                xreg += *imm;
            }
        }
    }

    grid
}

fn main() {
    let data = fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let lines = data.lines().collect::<Vec<&str>>();

    let instrs = lines
        .iter()
        .map(|inst| {
            if *inst == "noop" {
                Instruction::Noop
            } else {
                let (_, off) = inst.split_once(" ").unwrap();
                Instruction::AddX(off.parse::<i32>().unwrap())
            }
        })
        .collect::<Vec<Instruction>>();


    println!("> {}", find_xreg_sums(&instrs));
    
    let display = draw_crt(&instrs);
    for y in display {
        for x in y {
            print!("{}", x);
        }
        println!("");
    }
}
