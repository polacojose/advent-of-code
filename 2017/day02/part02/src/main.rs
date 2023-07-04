use std::fs;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut divs = Vec::new();

    for line in file_string.lines() {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut found = false;
        for first_num_index in 0..numbers.len() {
            let first_num = numbers[first_num_index];
            for second_num_index in first_num_index + 1..numbers.len() {
                let second_num = numbers[second_num_index];

                if first_num % second_num == 0 {
                    let div = first_num / second_num;
                    divs.push(div);
                    println!("Div: {first_num}/{second_num} = {}", div);
                    found = true;
                    break;
                }

                if second_num % first_num == 0 {
                    let div = second_num / first_num;
                    divs.push(div);
                    println!("Div: {second_num}/{first_num} = {}", div);
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }

    println!("Checksum: {}", divs.iter().sum::<usize>());
}
