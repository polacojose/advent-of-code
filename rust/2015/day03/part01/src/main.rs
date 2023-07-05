use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

fn main() {
    let file_string = fs::read_to_string("./input.txt").unwrap();

    for line in file_string.lines() {
        let mut location = Vector { x: 0, y: 0 };
        let mut unique_houses = HashSet::new();
        line.bytes().for_each(|b| {
            unique_houses.insert(location.clone());
            alter_vector_by_byte(&mut location, b);
        });
        println!("Houses visited {}", unique_houses.len());
    }
}

fn alter_vector_by_byte(vector: &mut Vector, byte: u8) {
    match byte {
        b'^' => vector.y += 1,
        b'v' => vector.y -= 1,
        b'<' => vector.x -= 1,
        b'>' => vector.x += 1,
        _ => (),
    }
}
