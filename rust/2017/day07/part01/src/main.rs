use std::{fs, str::FromStr};

#[derive(Debug)]
struct Tower {
    prefix: String,
    suffix: Vec<String>,
}

#[derive(Debug)]
struct UnableToParse;
impl FromStr for Tower {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((prefix, suffix)) = s.split_once("->") {
            Ok(Self {
                prefix: s.split_whitespace().next().unwrap().to_string(),
                suffix: suffix
                    .replace(",", "")
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
            })
        } else {
            Ok(Self {
                prefix: s.split_whitespace().next().unwrap().to_string(),
                suffix: vec![],
            })
        }
    }
}

fn main() {
    let towers = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<Tower>().unwrap())
        .collect::<Vec<_>>();

    for tower_a in &towers {
        let mut root = true;
        for tower_b in &towers {
            if tower_b.suffix.contains(&tower_a.prefix) {
                root = false;
                break;
            }
        }

        if root {
            println!("Root Tower: {:?}", tower_a);
        }
    }
}
