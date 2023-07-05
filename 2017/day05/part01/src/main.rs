use std::fs;

fn main() {
    let mut jumps = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    let mut ip: isize = 0;
    let mut steps = 0;
    while let Some(jump) = jumps.get_mut(ip as usize) {
        steps += 1;
        ip += *jump;
        *jump += 1;
    }

    println!("Steps: {}", steps);
}
