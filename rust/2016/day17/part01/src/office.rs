use crate::{astar::AStarVector, INITIAL_HASH};

const OFFICE_WIDTH: isize = 4;
const OFFICE_HEIGHT: isize = 4;

pub fn is_wall(x: isize, y: isize) -> bool {
    x < 0 || x > OFFICE_WIDTH || y < 0 || y > OFFICE_HEIGHT
}

pub fn display_output(max_x: usize, max_y: usize) -> String {
    let mut output_string = String::new();

    output_string.push_str(&format!("  "));
    for i in 0..=max_x {
        if max_x >= 10 {
            output_string.push_str(&format!("{:2}", i));
        } else {
            output_string.push_str(&format!("{}", i));
        }
    }
    output_string.push('\n');

    for y in 0..=max_y {
        if max_y >= 10 {
            output_string.push_str(&format!("{:2} ", y));
        } else {
            output_string.push_str(&format!("{} ", y));
        }
        for x in 0..=max_x {
            let cell = if is_wall(x as isize, y as isize) {
                "#"
            } else {
                "."
            };

            if max_x >= 10 {
                output_string.push_str(&format!("{:2}", cell));
            } else {
                output_string.push_str(&format!("{}", cell));
            }
        }
        output_string.push('\n');
    }

    output_string
}

pub fn display_output_marked(max_x: usize, max_y: usize, marked: Vec<AStarVector>) -> String {
    let mut output_string = String::new();

    output_string.push_str(&format!("  "));
    for i in 0..=max_x {
        if max_x >= 10 {
            output_string.push_str(&format!("{:2}", i));
        } else {
            output_string.push_str(&format!("{}", i));
        }
    }
    output_string.push('\n');

    for y in 0..=max_y {
        if max_y >= 10 {
            output_string.push_str(&format!("{:2} ", y));
        } else {
            output_string.push_str(&format!("{} ", y));
        }
        for x in 0..=max_x {
            let mut cell = if is_wall(x as isize, y as isize) {
                "#"
            } else {
                "."
            };

            if marked.contains(&AStarVector {
                door_hash: INITIAL_HASH.to_string(),
                f_score: None,
                x: x as isize,
                y: y as isize,
            }) {
                cell = "O";
            }

            if max_x >= 10 {
                output_string.push_str(&format!("{:2}", cell));
            } else {
                output_string.push_str(&format!("{}", cell));
            }
        }
        output_string.push('\n');
    }

    output_string
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_calc_output() {
        let correct_output = fs::read_to_string("demo-output.txt").unwrap();
        let test_output = display_output(9, 6);
        println!("{}", test_output);
        println!("{}", correct_output);
        assert_eq!(test_output, correct_output);
    }
}
