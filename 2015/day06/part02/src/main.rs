use std::fs;

use part2::{self, Command, Instruction};

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut light_grid: [[u64; 1000]; 1000] = [[0; 1000]; 1000];

    for line in file_string.lines() {
        let instruction = line.parse::<Instruction>().unwrap();

        execute_instruction(&instruction, &mut light_grid);

        // println!(
        //     "{} = {}",
        //     line,
        //     light_grid.iter().flatten().filter(|l| **l).count()
        // );
    }
    println!(
        "Total brightness {}",
        light_grid.iter().flatten().sum::<u64>()
    );

    print_light_grid(&light_grid);
}

fn execute_instruction(instruction: &Instruction, light_grid: &mut [[u64; 1000]; 1000]) {
    for y in instruction.rect.upper_left.y..=instruction.rect.lower_right.y {
        for x in instruction.rect.upper_left.x..=instruction.rect.lower_right.x {
            match instruction.command {
                Command::TurnOn => light_grid[x as usize][y as usize] += 1,
                Command::TurnOff => {
                    light_grid[x as usize][y as usize] =
                        light_grid[x as usize][y as usize].saturating_sub(1)
                }
                Command::Toggle => light_grid[x as usize][y as usize] += 2,
            }
        }
    }
}

fn print_light_grid(light_grid: &[[u64; 1000]; 1000]) {
    for (i, row) in light_grid.iter().enumerate() {
        if i % 10 == 0 {
            continue;
        }
        for (i, light) in row.iter().enumerate() {
            if i % 10 == 0 {
                continue;
            }
            print!("{}", light);
        }
    }
}
