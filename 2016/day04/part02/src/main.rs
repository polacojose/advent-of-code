use std::{collections::HashMap, fs, str::FromStr};

use regex::Regex;

#[derive(Debug)]
struct RoomCode {
    encrypted_name: String,
    sector_id: u16,
    check_sum: String,
}

#[derive(Debug)]
struct UnableToParse;
impl FromStr for RoomCode {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"(?x)
            (?P<encrypted_name>.*)-(?P<sector_id>\d{3})\[(?P<check_sum>\w{5})\]
            ",
        )
        .unwrap();
        let captures = re.captures(s).unwrap();

        Ok(RoomCode {
            encrypted_name: captures["encrypted_name"].to_string().to_lowercase(),
            sector_id: captures["sector_id"].parse().unwrap(),
            check_sum: captures["check_sum"].to_string(),
        })
    }
}

fn main() {
    let room_codes = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse::<RoomCode>().unwrap())
        .collect::<Vec<RoomCode>>();

    for room_code in room_codes {
        let valid = validate_check_sum(&room_code);
        if valid {
            decrypt_room_code(&room_code);
        }
    }
}

fn decrypt_room_code(room_code: &RoomCode) -> String {
    let mut decrypted_name = String::new();
    for c in room_code.encrypted_name.chars() {
        if c == '-' {
            decrypted_name.push(' ');
            continue;
        }
        decrypted_name.push(forward_character(&c, room_code.sector_id));
    }
    println!("{} -> {}", room_code.sector_id, decrypted_name);
    decrypted_name
}

fn forward_character(c: &char, forwards: u16) -> char {
    let mut forwarded_num = *c as u16 + forwards;
    while forwarded_num >= ('a' as u16) + 26 {
        forwarded_num -= 26;
    }
    char::from_u32(forwarded_num as u32).unwrap()
}

fn calculate_check_sum(encrypted_name: impl AsRef<str>) -> String {
    let chars = encrypted_name
        .as_ref()
        .replace("-", "")
        .chars()
        .collect::<Vec<_>>();

    let mut hash_map = HashMap::new();
    for c in chars {
        hash_map.entry(c).and_modify(|num| *num += 1).or_insert(1);
    }

    let mut check_sum = hash_map.into_iter().collect::<Vec<(_, _)>>();
    check_sum.sort_by(|a, b| a.0.cmp(&b.0));
    check_sum.sort_by(|a, b| b.1.cmp(&a.1));
    String::from_iter(check_sum.into_iter().map(|(a, _)| a).collect::<Vec<_>>())[..5].to_string()
}

fn validate_check_sum(room_code: &RoomCode) -> bool {
    let check_sum = calculate_check_sum(&room_code.encrypted_name);
    room_code.check_sum == check_sum
}
