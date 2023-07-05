use crate::astar::AStarVector;

pub const OFFICE_DESIGNERS_FAVORITE_NUMBER: usize = 1350;
pub const MAX_DISTANCE: f64 = 50.0;

pub fn is_wall(x: usize, y: usize, favorite_number: usize) -> bool {
    !(((x * x + 3 * x + 2 * x * y + y + y * y) + favorite_number).count_ones() % 2 == 0)
}

pub fn display_output(max_x: usize, max_y: usize, favorite_number: usize) -> String {
    let mut output_string = String::new();

    if max_x >= 10 {
        output_string.push_str(&format!(" "));
    } else {
        output_string.push_str(&format!("  "));
    }
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
            let cell = if is_wall(x, y, favorite_number) {
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

pub fn display_output_marked(
    max_x: usize,
    max_y: usize,
    favorite_number: usize,
    marked: &Vec<AStarVector>,
) -> String {
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
            let mut cell = if is_wall(x, y, favorite_number) {
                "#"
            } else {
                "."
            };

            if marked.contains(&AStarVector {
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
        let test_output = display_output(9, 6, 10);
        println!("{}", test_output);
        println!("{}", correct_output);
        assert_eq!(test_output, correct_output);
    }
}
