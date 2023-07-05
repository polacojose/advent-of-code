use std::{collections::HashMap, fs, thread::sleep, time::Duration};

#[derive(Debug, Clone, Copy)]
enum HEADING {
    Right = 0,
    Up = 1,
    Left = 2,
    Down = 3,
}

impl From<usize> for HEADING {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Right,
            1 => Self::Up,
            2 => Self::Left,
            3 => Self::Down,
            _ => panic!(),
        }
    }
}

impl HEADING {
    fn rotate(&self, rotation: usize) -> Self {
        let rotation = ((*self as usize) + rotation) % 4;
        rotation.into()
    }
}

#[derive(Debug)]
struct Vector {
    position: (isize, isize),
    heading: HEADING,
}

fn add_heading_to_position(heading: &HEADING, mut position: (isize, isize)) -> (isize, isize) {
    match heading {
        HEADING::Right => position.0 += 1,
        HEADING::Up => position.1 -= 1,
        HEADING::Left => position.0 -= 1,
        HEADING::Down => position.1 += 1,
    }
    position
}

impl Vector {
    fn step_forward(&mut self) {
        self.position = add_heading_to_position(&self.heading, self.position);
    }
    fn get_position_at_side(&self, side: &HEADING) -> (isize, isize) {
        add_heading_to_position(side, self.position)
    }
}

fn add_surrounding_positions(
    position: (isize, isize),
    pos_map: &HashMap<(isize, isize), usize>,
) -> usize {
    let mut sum = 0;
    for y in -1..=1 {
        for x in -1..=1 {
            let new_pos = (position.0 + x, position.1 + y);
            if let Some(val) = pos_map.get(&new_pos) {
                sum += val;
            }
        }
    }
    sum
}

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    for target_number in file_string.lines().map(|l| l.parse::<isize>().unwrap()) {
        let mut map: HashMap<_, usize> = HashMap::new();
        map.insert((0, 0), 1);

        let mut current_vector = Vector {
            position: (0, 0),
            heading: HEADING::Right,
        };
        for _ in 2.. {
            current_vector.step_forward();

            //calculate value
            let new_value = add_surrounding_positions(current_vector.position, &map);

            //add value
            map.insert(current_vector.position, new_value);
            println!("{:?}={:?}", current_vector.position, new_value);

            if new_value as isize > target_number {
                return;
            }

            let position_left =
                current_vector.get_position_at_side(&current_vector.heading.rotate(1));

            if !map.contains_key(&position_left) {
                current_vector.heading = current_vector.heading.rotate(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut rotation = HEADING::Left;
        for _ in 0..100 {
            rotation = rotation.rotate(1);
            println!("{:?}", rotation);
        }
    }
}
