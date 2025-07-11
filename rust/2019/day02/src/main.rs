use std::fs;

use intcode_program::IntCodeProgram;

mod intcode;
mod intcode_program;
mod string_reader;

fn main() {
    let mut program: Vec<u32> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|i| i.parse::<u32>().unwrap())
        .collect();

    part1(program.clone());
}

fn part1(mut program: Vec<u32>) {
    program[1] = 12;
    program[2] = 2;

    let mut intcode_program = IntCodeProgram::new(program);

    let result = intcode_program.execute();
    println!("{:?}", result[0]);
}
