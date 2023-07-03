use std::fs;

use part01::{gen_map, solve};

fn main() {
    let input_map = gen_map(fs::read_to_string("input.txt").unwrap());

    let result = solve(&input_map).unwrap();
    println!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_demo_search() {
        let input_map = gen_map(fs::read_to_string("demo-input.txt").unwrap());

        let result = solve(&input_map).unwrap();

        assert_eq!(result, 14);
    }
}
