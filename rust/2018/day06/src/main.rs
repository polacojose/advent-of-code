use std::fs;

use grid::Grid;

mod grid;

fn main() {
    let grid: Grid = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| {
            let (xx, yy) = s.split_once(",").unwrap();
            (
                xx.trim().parse::<usize>().unwrap(),
                yy.trim().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
        .into();

    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Grid) {
    println!("Largest Area: {:?}", grid.largest_area());
}

fn part2(grid: &Grid) {
    println!(
        "Central Location Area: {:?}",
        grid.central_location_area(10000)
    );
}
