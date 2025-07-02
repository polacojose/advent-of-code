use std::collections::HashMap;

use gapbuf::GapBuffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stone(pub u64);

impl Stone {
    pub fn blink(&self) -> Vec<Self> {
        match self.0 {
            0 => {
                vec![Stone(1)]
            }
            x if num_digits(x) % 2 == 0 => {
                let digit_half = 10_u64.pow(num_digits(x) / 2);
                let left = x / digit_half;
                let right = x - left * digit_half;
                vec![Stone(left), Stone(right)]
            }
            _ => {
                vec![Stone(self.0 * 2024)]
            }
        }
    }
}

pub fn blink_n(
    n: u32,
    stone: &Stone,
    stone_map: &mut HashMap<(u32, u64), Vec<Stone>>,
) -> Vec<Stone> {
    b_n(n - 1, stone, stone_map)
}

fn b_n(n: u32, stone: &Stone, stone_map: &mut HashMap<(u32, u64), Vec<Stone>>) -> Vec<Stone> {
    if let Some(rs) = stone_map.get(&(n, stone.0)) {
        return rs.clone();
    }

    if n == 0 {
        let result_stone = stone.blink();
        stone_map.insert((0, stone.0), result_stone.clone());
        return result_stone;
    }

    b_n(n - 1, stone, stone_map)
        .into_iter()
        .flat_map(|s| b_n(0, &s, stone_map))
        .collect()
}

fn num_digits(n: u64) -> u32 {
    let mut digits = 1;
    while n / 10_u64.pow(digits) > 0 {
        digits += 1;
    }
    digits
}

pub fn blink_stones(stones: &mut GapBuffer<Stone>) {
    let mut idx = 0;
    while idx < stones.len() {
        let stone_val = stones[idx].0;
        let num_digits = num_digits(stone_val);
        println!("{stone_val}");
        match stone_val {
            0 => stones[idx].0 = 1,
            x if num_digits % 2 == 0 => {
                let digit_half = 10_u64.pow(num_digits / 2);
                let left = x / digit_half;
                let right = x - left * digit_half;

                stones[idx].0 = left;
                stones.insert(idx + 1, Stone(right));

                idx += 1;
            }
            _ => stones[idx].0 *= 2024,
        }
        idx += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(1), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(100), 3);
    }

    #[test]
    fn test_blink() {
        assert_eq!(Stone(0).blink(), vec![Stone(1)]);
        assert_eq!(Stone(1).blink(), vec![Stone(2024)]);
        assert_eq!(Stone(10).blink(), vec![Stone(1), Stone(0)]);
        assert_eq!(Stone(99).blink(), vec![Stone(9), Stone(9)]);
        assert_eq!(Stone(999).blink(), vec![Stone(2021976)]);
    }

    macro_rules! test_blink_stones {
        ($stones:expr, $results:expr) => {
            blink_stones($stones);

            assert_eq!(
                *$stones,
                $results
                    .trim()
                    .split_whitespace()
                    .map(|s| Stone(s.trim().parse::<u64>().unwrap()))
                    .collect::<Vec<_>>()
            );
        };
    }

    #[test]
    fn test_blink_stones() {
        let mut stones = "125 17"
            .trim()
            .split_whitespace()
            .map(|s| Stone(s.trim().parse::<u64>().unwrap()))
            .collect::<GapBuffer<_>>();
        test_blink_stones!(&mut stones, "253000 1 7");
        test_blink_stones!(&mut stones, "253 0 2024 14168");
        test_blink_stones!(&mut stones, "512072 1 20 24 28676032");
        test_blink_stones!(&mut stones, "512 72 2024 2 0 2 4 2867 6032");
        test_blink_stones!(
            &mut stones,
            "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"
        );
        test_blink_stones!(
            &mut stones,
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"
        );
    }

    #[test]
    fn test_blink_stones_count() {
        let mut stones = "125 17"
            .trim()
            .split_whitespace()
            .map(|s| Stone(s.trim().parse::<u64>().unwrap()))
            .collect::<GapBuffer<_>>();

        for _ in 0..25 {
            blink_stones(&mut stones);
        }

        assert_eq!(stones.len(), 55312);
    }

    #[test]
    fn test_blink_stones_n() {
        let stones = "125 17"
            .trim()
            .split_whitespace()
            .map(|s| Stone(s.trim().parse::<u64>().unwrap()))
            .collect::<GapBuffer<_>>();

        let mut stone_map = HashMap::new();
        let total_stones = stones
            .into_iter()
            .flat_map(|s| blink_n(25, &s, &mut stone_map))
            .collect::<Vec<_>>();

        assert_eq!(total_stones.len(), 55312);
    }
}
