use std::{fs, str::FromStr};

use regex::Regex;

#[derive(Debug)]
struct IPv7 {
    address: String,
    supernets: Vec<String>,
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

        let mut supernets = s.clone().to_string();
        for hypernet in &hypernets {
            supernets = supernets.replace(format!("[{}]", hypernet).as_str(), " ");
        }
        let supernets = supernets.split(" ").map(|s| s.to_owned()).collect();

        Ok(IPv7 {
            address: s.to_string(),
            supernets,
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

fn get_abas(ip: &IPv7) -> Vec<(char, char)> {
    let mut abas = Vec::new();
    for supernet in &ip.supernets {
        let chars = supernet.chars().collect::<Vec<char>>();
        for i in 2..chars.len() {
            if chars[i - 2] == chars[i] && chars[i - 1] != chars[i] {
                abas.push((chars[i - 2], chars[i - 1]));
            }
        }
    }
    abas
}

fn has_babs(ip: &IPv7, abas: Vec<(char, char)>) -> bool {
    for aba in abas {
        for hypernet in &ip.hypernets {
            let chars = hypernet.chars().collect::<Vec<char>>();
            for i in 2..chars.len() {
                if chars[i - 2] == aba.1 && chars[i - 1] == aba.0 && chars[i] == aba.1 {
                    return true;
                }
            }
        }
    }

    false
}

fn check_ip_ssl(ip: &IPv7) -> bool {
    has_babs(ip, get_abas(ip))
}

fn main() {
    let ip_addresses = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<IPv7>().unwrap())
        .collect::<Vec<IPv7>>();

    let number_supported = ip_addresses.iter().filter(|ip| check_ip_ssl(ip)).count();

    println!("{:#?}", number_supported);
}
