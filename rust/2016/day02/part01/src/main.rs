use std::{fs, str::FromStr};

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug)]
struct UnableToParse;
impl FromStr for Direction {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase().chars().collect::<Vec<_>>();
        match s[0] {
            'u' => Ok(Direction::Up),
            'l' => Ok(Direction::Left),
            'r' => Ok(Direction::Right),
            'd' => Ok(Direction::Down),
            _ => Err(UnableToParse),
        }
    }
}

struct ClampedCoord {
    x: i32,
    y: i32,
}

impl ClampedCoord {
    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
        }

        self.x = self.x.max(-1);
        self.x = self.x.min(1);
        self.y = self.y.max(-1);
        self.y = self.y.min(1);
    }
}

#[inline]
fn get_keypad_num(x: i8, y: i8) -> u8 {
    assert!(-1 <= x && x <= 1);
    assert!(-1 <= y && y <= 1);

    let x = x + 2;
    let y = y + 2;

    (y * 3 + (x - 3)) as u8
}

fn main() {
    let files_string = fs::read_to_string("input.txt").unwrap();

    let mut keypad = ClampedCoord { x: 0, y: 0 };
    for line in files_string.lines() {
        let directions = line
            .chars()
            .map(|x| x.to_string().parse::<Direction>().unwrap())
            .collect::<Vec<_>>();

        for direction in directions {
            keypad.move_direction(direction);
        }

        let number = get_keypad_num(keypad.x as i8, keypad.y as i8);
        print!("{}", number);
    }
    println!()
}
