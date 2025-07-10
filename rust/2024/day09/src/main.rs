use std::fs;

use day09::disk::disk::Disk;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut disk = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Disk>()
        .unwrap();

    disk.frag_compact();
    println!("Part1: {}", disk.checksum());
}

fn part2() {
    let mut disk = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Disk>()
        .unwrap();

    disk.quick_defrag();
    println!("Part2: {}", disk.checksum());
}
