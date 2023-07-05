use std::fs;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut character_count = 0;

    let mut escaped_character_count = 0;
    for line in file_string.lines() {
        let line = line.trim();
        character_count += line.len();

        let escaped_length = escaped_filter_line(line);
        escaped_character_count += escaped_length;
    }

    println!("Escaped Character Count: {}", escaped_character_count);
    println!("Character Count: {}", character_count);
    println!("Difference {}", escaped_character_count - character_count);
}

fn escaped_filter_line(line: &str) -> usize {
    let mut line_bytes = line.bytes().collect::<Vec<u8>>();

    let mut count = 2;
    while line_bytes.len() > 0 {
        let byte = line_bytes.remove(0);
        if byte == b'\\' {
            count += 2;
            continue;
        }

        if byte == b'\"' {
            count += 2;
            continue;
        }
        count += 1;
    }
    count
}
