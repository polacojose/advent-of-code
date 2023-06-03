use std::fs;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut character_count = 0;

    let mut memory_count = 0;
    for line in file_string.lines() {
        let line = line.trim();
        character_count += line.len();

        memory_count += memory_filter_line(line);
    }

    println!("Character Count: {}", character_count);
    println!("Memory Count: {}", memory_count);
    println!("Difference {}", character_count - memory_count);
}

fn memory_filter_line(line: &str) -> usize {
    let line = line[1..line.len() - 1].to_owned();

    let mut line_bytes = line.bytes().collect::<Vec<u8>>();

    let mut count = 0;
    while line_bytes.len() > 0 {
        let byte = line_bytes.remove(0);
        if byte == b'\\' {
            if line_bytes[0] == b'\\' {
                line_bytes.remove(0);
                count += 1;
            } else if line_bytes[0] == b'x' {
                line_bytes.remove(0);
                line_bytes.remove(0);
                line_bytes.remove(0);
                count += 1;
            }
        } else {
            count += 1;
        }
    }
    count
}
