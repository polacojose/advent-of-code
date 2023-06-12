use std::{fs, os::fd, str::FromStr};

use regex::Regex;

#[derive(Debug)]
struct IPv7 {
    address: String,
    non_hypernets: String,
    hypernets: Vec<String>,
}

#[derive(Debug)]
struct UnableToParse;
impl FromStr for IPv7 {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\[(\w+)\]").unwrap();

        let hypernets = re
            .captures_iter(s)
            .map(|x| x[1].to_string())
            .collect::<Vec<String>>();

        let mut non_hypernets = s.clone().to_string();
        for hypernet in &hypernets {
            non_hypernets = non_hypernets.replace(format!("[{}]", hypernet).as_str(), " ");
        }

        Ok(IPv7 {
            address: s.to_string(),
            non_hypernets,
            hypernets,
        })
    }
}

fn check_sequence_abba(s: impl AsRef<str>) -> bool {
    let chars = s.as_ref().chars().collect::<Vec<char>>();
    for i in 3..chars.len() {
        if chars[i - 3] == chars[i] && chars[i - 2] == chars[i - 1] && chars[i - 1] != chars[i] {
            return true;
        }
    }

    false
}

fn check_ip_abba(ip: &IPv7) -> bool {
    if !check_sequence_abba(&ip.non_hypernets) {
        return false;
    }

    for hypernet in &ip.hypernets {
        if check_sequence_abba(hypernet) {
            return false;
        }
    }

    true
}

fn main() {
    let ip_addresses = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<IPv7>().unwrap())
        .collect::<Vec<IPv7>>();

    let number_supported = ip_addresses.iter().filter(|ip| check_ip_abba(ip)).count();

    println!("{:#?}", number_supported);
}
