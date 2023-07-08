use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    let dense_hash = get_dense_hash(input);
    println!("{}", dense_hash);
}

fn get_dense_hash(input: impl Into<String>) -> String {
    let mut lengths: Vec<u8> = input.into().into_bytes();
    for s in [17, 31, 73, 47, 23].into_iter() {
        lengths.push(s);
    }

    let mut sparse_hash = get_list(256);

    let mut start_pos = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        for length in &lengths {
            sparse_hash.rotate_left(start_pos);

            let section = sparse_hash.drain(0..*length as usize).collect::<Vec<_>>();
            for n in section {
                sparse_hash.push_front(n)
            }

            sparse_hash.rotate_right(start_pos);
            start_pos += *length as usize + skip_size;
            start_pos = start_pos % sparse_hash.len();
            skip_size += 1;
        }
    }

    let dense_hash = sparse_hash
        .into_iter()
        .collect::<Vec<_>>()
        .chunks(16)
        .map(|chunk| chunk.into_iter().cloned().reduce(|a, b| a ^ b).unwrap())
        .collect::<Vec<_>>();

    let dense_hash_hex = dense_hash
        .into_iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();

    dense_hash_hex
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
        let dense_hash = get_dense_hash("");
        assert_eq!(dense_hash, "a2582a3a0e66e6e86e3812dcb672a272");

        let dense_hash = get_dense_hash("AoC 2017");
        assert_eq!(dense_hash, "33efeb34ea91902bb2f59c9920caa6cd");

        let dense_hash = get_dense_hash("1,2,3");
        assert_eq!(dense_hash, "3efbe78a8d82f29979031a4aa0b16a9d");

        let dense_hash = get_dense_hash("1,2,4");
        assert_eq!(dense_hash, "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
