use std::fs;

use part01::nodes::nodegrid::NodeGrid;

fn main() {
    let node_grid: NodeGrid = fs::read_to_string("input.txt").unwrap().parse().unwrap();

    part1(&node_grid);
    part2();
}

fn part1(node_grid: &NodeGrid) {
    println!("Viable pairs: {}", node_grid.viable_pairs().len())
}

fn part2() {}
