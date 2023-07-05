use std::fs;

fn main() {
    //read file
    let file_string = fs::read_to_string("./input.txt").unwrap();

    let mut current_floor: i64 = 0;
    let basement_entry_position = file_string
        .bytes()
        .map(|b| match b {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .position(|x| {
            current_floor += x;
            if current_floor == -1 {
                true
            } else {
                false
            }
        })
        .unwrap()
        + 1;

    println!("basement_entry_position: {}", basement_entry_position);
}
