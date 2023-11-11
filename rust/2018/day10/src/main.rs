use std::{fs, thread::sleep, time::Duration};

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

    let mut grid = StarGrid::new(40, vectors);

    let mut seconds = 1;
    loop {
        grid.delta(1);
        if grid.x_divider() < 3.0 {
            println!("Seconds: {}", seconds);
            println!("{}", grid.output());
            sleep(Duration::from_millis(1000));
        }
        seconds += 1;
    }
}
