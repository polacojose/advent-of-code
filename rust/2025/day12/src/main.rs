use day12::plot::{PlotNode, parameter_price_map, side_price_map};
use grid::grid::map::Grid;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let plot_grid = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Grid<PlotNode>>()
        .unwrap();
    let price = parameter_price_map(&plot_grid);
    println!("Part1: {price}");
}

fn part2() {
    let plot_grid = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Grid<PlotNode>>()
        .unwrap();
    let price = side_price_map(&plot_grid);
    println!("Part2: {price}");
}
