use std::fs;

use grid::StarGrid;
use vector::Vector;

mod grid;
mod vector;

fn main() {
    let vectors = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse::<Vector>().unwrap())
        .collect();

    let mut grid = StarGrid::new(65, 10, vectors);

    let mut seconds = 1;
    let mut last_divider = f64::INFINITY;
    let mut last_grid_output = String::new();
    loop {
        grid.delta(1);

        if grid.x_divider() > last_divider {
            println!("Seconds: {}", seconds);
            println!("{}\n", last_grid_output);
            break;
        }
        last_divider = grid.x_divider();
        last_grid_output = grid.output();

        seconds += 1;
    }
}
