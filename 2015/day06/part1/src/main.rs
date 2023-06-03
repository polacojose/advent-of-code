use std::fs;

use part1::{self, Command, Instruction};

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut light_grid = [[false; 1000]; 1000];

    for line in file_string.lines() {
        let instruction = line.parse::<Instruction>().unwrap();

        execute_instruction(&instruction, &mut light_grid);

        // println!(
        //     "{} = {}",
        //     line,
        //     light_grid.iter().flatten().filter(|l| **l).count()
        // );
    }
    println!("{}", light_grid.iter().flatten().filter(|l| **l).count());
}

fn execute_instruction(instruction: &Instruction, light_grid: &mut [[bool; 1000]; 1000]) {
    for y in instruction.rect.upper_left.y..=instruction.rect.lower_right.y {
        for x in instruction.rect.upper_left.x..=instruction.rect.lower_right.x {
            match instruction.command {
                Command::TurnOn => light_grid[x as usize][y as usize] = true,
                Command::TurnOff => light_grid[x as usize][y as usize] = false,
                Command::Toggle => {
                    light_grid[x as usize][y as usize] = !light_grid[x as usize][y as usize]
                }
            }
        }
    }
}
