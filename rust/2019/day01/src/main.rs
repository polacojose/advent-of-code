use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let total_fuel: u32 = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| calculate_fuel(line.parse::<u32>().unwrap()))
        .sum();

    println!("Total Fuel: {}", total_fuel);
}

fn part2() {
    let total_fuel: u32 = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| calculate_fuel_with_fuel_mass(line.parse::<u32>().unwrap()))
        .sum();

    println!("Total Fuel with Fuel Mass: {}", total_fuel);
}

fn calculate_fuel(mass: u32) -> u32 {
    mass / 3 - 2
}

fn calculate_fuel_with_fuel_mass(mass: u32) -> u32 {
    let fuel_mass = (mass / 3).saturating_sub(2);

    if fuel_mass == 0 {
        return 0;
    }
    fuel_mass + calculate_fuel_with_fuel_mass(fuel_mass)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_calculate_fuel_without_fuel_mass() {
        assert_eq!(2, calculate_fuel(12));
        assert_eq!(2, calculate_fuel(14));
        assert_eq!(654, calculate_fuel(1969));
        assert_eq!(33583, calculate_fuel(100756));
    }

    #[test]
    fn can_calculate_fuel_with_fuel_mass() {
        assert_eq!(2, calculate_fuel_with_fuel_mass(14));
        assert_eq!(966, calculate_fuel_with_fuel_mass(1969));
        assert_eq!(50346, calculate_fuel_with_fuel_mass(100756));
    }
}
