use std::fs;

use day10::{
    gridnode::GridNode,
    search::{get_node_peaks, get_trail_counts},
};
use grid::grid::map::Grid;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut grid = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Grid<GridNode>>()
        .unwrap();
    let trail_heads = grid
        .get_nodes()
        .iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if n.height == 0 {
                Some(grid.index_to_vector(i)?)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let peak_count = trail_heads
        .iter()
        .map(|v| get_node_peaks(&mut grid, v).len())
        .sum::<usize>();

    println!("Part1: {peak_count}");
}

fn part2() {
    let mut grid = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Grid<GridNode>>()
        .unwrap();
    let trail_heads = grid
        .get_nodes()
        .iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if n.height == 0 {
                Some(grid.index_to_vector(i)?)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let peak_count = trail_heads
        .iter()
        .map(|v| get_trail_counts(&mut grid, v))
        .sum::<usize>();

    println!("Part2: {peak_count}");
}
