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
    fn validate(&self, num: usize) -> Result<bool, RangeInclusive<usize>> {
        for range in &self.blacklist {
            if range.contains(&num) {
                return Err(range.clone());
            }
        }
        Ok(true)
    }
}

fn main() {
    let range_validation: RangeValidation =
        fs::read_to_string("input.txt").unwrap().parse().unwrap();

    let mut allowed_ips = Vec::new();
    let mut ip = 0;
    while ip < u32::MAX as usize {
        match range_validation.validate(ip) {
            Ok(_) => {
                allowed_ips.push(ip);
                ip += 1;
            }
            Err(range) => {
                ip = *range.end() + 1;
            }
        }
    }
    println!("Allowed number of ips: {}", allowed_ips.len());
}
