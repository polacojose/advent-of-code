use std::{collections::HashMap, fs, str::FromStr};

use lazy_static::lazy_static;

#[derive(Clone, Copy)]
struct FirewallScanner {
    depth: usize,
    range: usize,
    location: usize,
}

#[derive(Debug)]
struct UnableToParse;
impl FromStr for FirewallScanner {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(":").map(|s| s.trim()).collect::<Vec<_>>();

        Ok(Self {
            depth: parts[0].parse().unwrap(),
            range: parts[1].parse().unwrap(),
            location: 0,
        })
    }
}

impl FirewallScanner {
    pub fn cost(&self, time: usize) -> usize {
        let location = if self.range >= 2 {
            time % ((self.range * 2) - 2)
        } else {
            0
        };

        if location == 0 {
            return self.depth * self.range;
        }

        0
    }
}

fn get_scanner(depth: usize) -> FirewallScanner {
    if let Some(fs) = FIREWALL_SCANNERS.get(&depth) {
        return fs.clone();
    }

    return FirewallScanner {
        depth: 0,
        range: 0,
        location: 0,
    };
}

#[inline]
fn get_scanner_length() -> usize {
    FIREWALL_SCANNERS.keys().max().cloned().unwrap_or(0)
}

lazy_static! {
    static ref FIREWALL_SCANNERS: HashMap<usize, FirewallScanner> = {
        let mut map = HashMap::new();
        fs::read_to_string("input.txt")
            .unwrap()
            .lines()
            .map(|l| l.parse::<FirewallScanner>().unwrap())
            .into_iter()
            .for_each(|fs| {
                map.insert(fs.depth, fs);
            });
        map
    };
}

fn main() {
    let mut total_cost = 0;
    for i in 0..=get_scanner_length() {
        println!("Depth: {} | Cost: {}", i, get_scanner(i).cost(i));
        total_cost += get_scanner(i).cost(i);
    }
    println!("{}", total_cost)
}
