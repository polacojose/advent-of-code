use std::fs;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut nice_strings = 0;
    for line in file_string.lines() {
        let line = line.trim();

        if is_nice_string(line) {
            nice_strings += 1;
            println!("{} is nice", line);
        } else {
            println!("{} is naughty", line);
        }
    }
    println!("{} nice strings", nice_strings);
}

fn is_nice_string(string: impl AsRef<str>) -> bool {
    let s = string.as_ref();
    if !has_magic_three(s) {
        return false;
    }

    if !has_double_pair(s) {
        return false;
    }

    return true;
}

fn has_magic_three(s: &str) -> bool {
    let s = s.as_bytes();
    for i in 2..s.len() {
        if s[i - 2] == s[i] {
            return true;
        }
    }

    false
}

fn has_double_pair(s: &str) -> bool {
    let s = s.as_bytes();
    for i in 2..s.len() {
        if s[i..s.len()]
            .windows(2)
            .position(|w| [w[0], w[1]] == [s[i - 2], s[i - 1]])
            .is_some()
        {
            return true;
        }
    }

    false
}
