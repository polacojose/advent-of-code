use std::{collections::VecDeque, fs};

fn main() {
    let lengths = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .into_iter()
        .map(|x| {
            x.replace(" ", "")
                .split(",")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .flatten()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut list = get_list(256);

    let mut start_pos = 0;
    let mut skip_size = 0;

    for length in lengths {
        list.rotate_left(start_pos);

        let section = list.drain(0..length).collect::<Vec<_>>();
        for n in section {
            list.push_front(n)
        }

        list.rotate_right(start_pos);
        start_pos += length + skip_size;
        start_pos = start_pos % list.len();
        skip_size += 1;
    }
    let check = list
        .range(0..2)
        .into_iter()
        .cloned()
        .reduce(|a, b| a * b)
        .unwrap();
    println!("Check: {:?}", check);
}

fn get_list(size: usize) -> VecDeque<usize> {
    let mut list = VecDeque::with_capacity(size);
    for i in 0..size {
        list.push_back(i);
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lengths = fs::read_to_string("demo-input.txt")
            .unwrap()
            .lines()
            .into_iter()
            .map(|x| {
                x.replace(" ", "")
                    .split(",")
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();

        let mut start_pos = 0;
        let mut skip_size = 0;

        let mut list = get_list(5);
        for length in lengths {
            list.rotate_left(start_pos);

            let section = list.drain(0..length).collect::<Vec<_>>();
            for n in section {
                list.push_front(n)
            }

            list.rotate_right(start_pos);
            start_pos += length + skip_size;
            start_pos = start_pos % list.len();
            skip_size += 1;
        }

        let check = list
            .range(0..2)
            .into_iter()
            .cloned()
            .reduce(|a, b| a * b)
            .unwrap();
        println!("Check: {:?}", check);

        assert_eq!(check, 12)
    }
}
