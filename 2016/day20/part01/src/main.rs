use std::{fs, ops::RangeInclusive, str::FromStr};

#[derive(Debug)]
struct RangeValidation {
    blacklist: Vec<RangeInclusive<usize>>,
}

#[derive(Debug)]
struct UnableToParse;
impl FromStr for RangeValidation {
    type Err = UnableToParse;

    fn from_str(file: &str) -> Result<Self, Self::Err> {
        let mut ranges = Vec::new();
        for line in file.lines() {
            let parts = line.trim().split("-").collect::<Vec<_>>();
            ranges.push((parts[0].parse::<usize>().unwrap())..=(parts[1].parse::<usize>().unwrap()))
        }
        Ok(Self { blacklist: ranges })
    }
}

impl RangeValidation {
    fn validate(&self, num: usize) -> bool {
        for range in &self.blacklist {
            if range.contains(&num) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let range_validation: RangeValidation =
        fs::read_to_string("input.txt").unwrap().parse().unwrap();

    for i in 0.. {
        if range_validation.validate(i) {
            println!("{}", i);
            return;
        }
    }
}
