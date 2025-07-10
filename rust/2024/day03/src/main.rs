use std::{fs, io::Read};

use day03::mulreader::MulReader;

fn main() {
    let mut r = fs::File::open("input.txt").unwrap();
    part1(&mut r);

    let mut r = fs::File::open("input.txt").unwrap();
    part2(&mut r);
}

fn part1<R>(r: &mut R)
where
    R: Read,
{
    let mul_reader = MulReader::new(r, true);
    let sum = mul_reader.into_iter().map(|m| m.solve()).sum::<i64>();
    println!("Part1: {sum}");
}

fn part2<R>(r: &mut R)
where
    R: Read,
{
    let mul_reader = MulReader::new(r, false);
    let sum = mul_reader.into_iter().map(|m| m.solve()).sum::<i64>();
    println!("Part1: {sum}");
}
