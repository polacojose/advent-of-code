use std::{str::FromStr, sync::RwLock};

use rustler::{nif, Env, NifStruct, NifUnitEnum, ResourceArc, Term};

#[derive(NifUnitEnum)]
enum InstructionType {
    On,
    Off,
    Toggle,
}

#[derive(NifStruct)]
#[module = "Instruction"]
struct Instruction {
    i_type: InstructionType,
    from: (usize, usize),
    to: (usize, usize),
}

#[derive(Debug)]
struct UnableToParseInstruction;
impl FromStr for Instruction {
    type Err = UnableToParseInstruction;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        Ok(match parts[1] {
            "on" => {
                let (x, y) = parts[2].split_once(",").unwrap();
                let (xx, yy) = parts[4].split_once(",").unwrap();
                Instruction {
                    i_type: InstructionType::On,
                    from: (x.parse().unwrap(), y.parse().unwrap()),
                    to: (xx.parse().unwrap(), yy.parse().unwrap()),
                }
            }
            "off" => {
                let (x, y) = parts[2].split_once(",").unwrap();
                let (xx, yy) = parts[4].split_once(",").unwrap();
                Instruction {
                    i_type: InstructionType::Off,
                    from: (x.parse().unwrap(), y.parse().unwrap()),
                    to: (xx.parse().unwrap(), yy.parse().unwrap()),
                }
            }
            _ => {
                let (x, y) = parts[1].split_once(",").unwrap();
                let (xx, yy) = parts[3].split_once(",").unwrap();
                Instruction {
                    i_type: InstructionType::Toggle,
                    from: (x.parse().unwrap(), y.parse().unwrap()),
                    to: (xx.parse().unwrap(), yy.parse().unwrap()),
                }
            }
        })
    }
}

pub struct GridPart1 {
    content: RwLock<Vec<Vec<bool>>>,
}

pub struct GridPart2 {
    content: RwLock<Vec<Vec<u64>>>,
}

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(GridPart1, env);
    rustler::resource!(GridPart2, env);
    rustler::resource!(Instruction, env);
    true
}

#[rustler::nif]
fn new_lights_part1(width: i64, height: i64, val: bool) -> ResourceArc<GridPart1> {
    return ResourceArc::new(GridPart1 {
        content: RwLock::new(vec![vec![val; width as usize]; height as usize]),
    });
}

#[rustler::nif]
fn grid_content_part1(grid: ResourceArc<GridPart1>) -> Vec<Vec<bool>> {
    return (*grid).content.read().unwrap().to_vec();
}

#[rustler::nif]
fn new_lights_part2(width: i64, height: i64, val: u64) -> ResourceArc<GridPart2> {
    return ResourceArc::new(GridPart2 {
        content: RwLock::new(vec![vec![val; width as usize]; height as usize]),
    });
}

#[rustler::nif]
fn grid_content_part2(grid: ResourceArc<GridPart2>) -> Vec<Vec<u64>> {
    return (*grid).content.read().unwrap().to_vec();
}

#[rustler::nif]
fn parse_instruction(line: &str) -> Instruction {
    return line.parse().unwrap();
}

#[nif(schedule = "DirtyCpu")]
fn execute_instruction_part1(
    grid: ResourceArc<GridPart1>,
    instruction: Instruction,
) -> ResourceArc<GridPart1> {
    {
        let mut content = (*grid).content.write().unwrap();
        match instruction.i_type {
            InstructionType::On => {
                for y in instruction.from.1..=instruction.to.1 {
                    for x in instruction.from.0..=instruction.to.0 {
                        content[y][x] = true;
                    }
                }
            }
            InstructionType::Off => {
                for y in instruction.from.1..=instruction.to.1 {
                    for x in instruction.from.0..=instruction.to.0 {
                        content[y][x] = false;
                    }
                }
            }
            InstructionType::Toggle => {
                for y in instruction.from.1..=instruction.to.1 {
                    for x in instruction.from.0..=instruction.to.0 {
                        content[y][x] = !content[y][x];
                    }
                }
            }
        };
    }

    return grid;
}

#[nif(schedule = "DirtyCpu")]
fn execute_instruction_part2(
    grid: ResourceArc<GridPart2>,
    instruction: Instruction,
) -> ResourceArc<GridPart2> {
    {
        let mut content = (*grid).content.write().unwrap();
        match instruction.i_type {
            InstructionType::On => {
                for y in instruction.from.1..=instruction.to.1 {
                    for x in instruction.from.0..=instruction.to.0 {
                        content[y][x] += 1;
                    }
                }
            }
            InstructionType::Off => {
                for y in instruction.from.1..=instruction.to.1 {
                    for x in instruction.from.0..=instruction.to.0 {
                        if content[y][x] > 0 {
                            content[y][x] -= 1;
                        }
                    }
                }
            }
            InstructionType::Toggle => {
                for y in instruction.from.1..=instruction.to.1 {
                    for x in instruction.from.0..=instruction.to.0 {
                        content[y][x] += 2;
                    }
                }
            }
        };
    }

    return grid;
}

rustler::init!(
    "Elixir.NativeVariant",
    [
        new_lights_part1,
        new_lights_part2,
        grid_content_part1,
        grid_content_part2,
        parse_instruction,
        execute_instruction_part1,
        execute_instruction_part2
    ],
    load = load
);
