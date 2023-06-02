use std::fs;

fn main() {
    //read file
    let file_string = fs::read_to_string("./input.txt").unwrap();

    let floor = file_string
        .bytes()
        .map(|b| match b {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum::<i64>();
    println!("floor: {}", floor);
}
