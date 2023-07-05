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
    if !has_three_vowels(s) {
        return false;
    }

    if !has_double_letter(s) {
        return false;
    }

    if has_bad_pairs(s) {
        return false;
    }

    return true;
}

fn has_three_vowels(s: &str) -> bool {
    let vowels = vec![b'a', b'e', b'i', b'o', b'u'];

    let mut num_vowels = 0;
    for byte in s.as_bytes() {
        if vowels.contains(byte) {
            num_vowels += 1;
        }

        if num_vowels >= 3 {
            return true;
        }
    }

    return false;
}

fn has_double_letter(s: &str) -> bool {
    s.as_bytes()
        .to_vec()
        .windows(2)
        .position(|w| w[0] == w[1])
        .is_some()
}

fn has_bad_pairs(s: &str) -> bool {
    let bad_pairs = vec![[b'a', b'b'], [b'c', b'd'], [b'p', b'q'], [b'x', b'y']];
    s.as_bytes()
        .to_vec()
        .windows(2)
        .position(|w| bad_pairs.to_vec().contains(&[w[0], w[1]]))
        .is_some()
}
