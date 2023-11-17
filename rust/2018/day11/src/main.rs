use crate::fuel_cell_grid::FuelCellGrid;

const GRID_SERIAL_NUMBER: i32 = 2694;

mod fuel_cell_grid;

fn main() {
    part1();
    part2();
}

fn part1() {
    let grid = FuelCellGrid::new(GRID_SERIAL_NUMBER);

    println!("Highest Power 3x3: {:?}", grid.highest_power_rect((3, 3)));
}

fn part2() {
    let result = FuelCellGrid::new(GRID_SERIAL_NUMBER).highest_power_rect_flex_rect();

    println!("Highest Power Flexible: {:?}", result);
}
