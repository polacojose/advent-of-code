use std::{fs, str::FromStr, sync::Mutex, thread::sleep, time::Duration};

use lazy_static::lazy_static;

#[derive(Debug)]
enum Instruction {
    Rect { x: usize, y: usize },
    RotateColumn { x: usize, r: usize },
    RotateRow { y: usize, r: usize },
}

#[derive(Debug)]
struct UnabletoParse;
impl FromStr for Instruction {
    type Err = UnabletoParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").map(|s| s.to_owned()).collect::<Vec<String>>();

        if parts.len() == 2 {
            let (x, y) = parts[1].split_once("x").unwrap();
            return Ok(Self::Rect {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            });
        }

        if parts.len() == 5 {
            let (_, dir) = parts[2].split_once("=").unwrap();
            match parts[1].as_str() {
                "column" => {
                    return Ok(Self::RotateColumn {
                        x: dir.parse().unwrap(),
                        r: parts[4].parse().unwrap(),
                    })
                }
                "row" => {
                    return Ok(Self::RotateRow {
                        y: dir.parse().unwrap(),
                        r: parts[4].parse().unwrap(),
                    })
                }
                _ => {}
            }
        }

        Err(UnabletoParse)
    }
}

impl Instruction {
    fn execute(&self) {
        match self {
            Self::Rect { x, y } => {
                for y in 0..*y {
                    for x in 0..*x {
                        let mut screen = SCREEN.lock().unwrap();
                        screen[x as usize][y as usize] = true;
                    }
                }
            }
            Self::RotateColumn { x, r } => {
                let mut screen = SCREEN.lock().unwrap();
                for _ in 0..*r {
                    let last = screen[*x][SCREEN_HEIGHT - 1];
                    for y in (1..SCREEN_HEIGHT).rev() {
                        screen[*x][y] = screen[*x][y - 1];
                    }
                    screen[*x][0] = last;
                }
            }
            Self::RotateRow { y, r } => {
                for _ in 0..*r {
                    {
                        let mut screen = SCREEN.lock().unwrap();
                        let last = screen[SCREEN_WIDTH - 1][*y];
                        for x in (1..SCREEN_WIDTH).rev() {
                            screen[x][*y] = screen[x - 1][*y];
                        }
                        screen[0][*y] = last;
                    }
                }
            }
        }
    }
}

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

lazy_static! {
    static ref SCREEN: Mutex<[[bool; SCREEN_HEIGHT]; SCREEN_WIDTH]> =
        Mutex::new([[false; SCREEN_HEIGHT]; SCREEN_WIDTH]);
}

fn main() {
    let instructions: Vec<Instruction> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    for instruction in &instructions {
        instruction.execute();
        sleep(Duration::from_millis(50));
        print!("{}[2J", 27 as char);
        println!("{:?}", instruction);
        print_screen();
    }

    println!(
        "{}",
        SCREEN
            .lock()
            .unwrap()
            .iter()
            .flatten()
            .filter(|x| **x)
            .count()
    );
}

fn print_screen() {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            print!(
                "{}",
                if SCREEN.lock().unwrap()[x][y] {
                    "#"
                } else {
                    "."
                }
            )
        }
        println!();
    }
}
