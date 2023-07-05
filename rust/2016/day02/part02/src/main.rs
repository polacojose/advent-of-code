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
    fn validate_coord(coord: (i32, i32)) -> bool {
        let (x, y) = coord;

        if coord == (0, -2) || coord == (0, 2) {
            return true;
        }

        if y >= -1 && y <= 1 && x >= -1 && x <= 1 {
            return true;
        }

        if y == 0 && x >= -2 && x <= 2 {
            return true;
        }

        false
    }
    fn move_direction(&mut self, direction: Direction) {
        let new_coord = match direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
            Direction::Down => (self.x, self.y + 1),
        };

        if ClampedCoord::validate_coord(new_coord) {
            self.x = new_coord.0;
            self.y = new_coord.1;
        }
    }
}

#[inline]
fn get_keypad_num(x: i8, y: i8) -> String {
    if y == -2 {
        return 1.to_string();
    }

    if y == 2 {
        return 'D'.to_string();
    }

    if y == -1 {
        return (x + 3).to_string();
    }

    if y == 1 {
        return std::char::from_u32(('B' as i8 + x) as u32)
            .unwrap()
            .to_string();
    }

    if y == 0 {
        return (x + 7).to_string();
    }

    panic!("Incorrect coord");
}

fn main() {
    let files_string = fs::read_to_string("input.txt").unwrap();

    let mut keypad = ClampedCoord { x: -2, y: 0 };
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
