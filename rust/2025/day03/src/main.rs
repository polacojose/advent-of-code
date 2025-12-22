use std::{fs, str::FromStr};

#[derive(Debug)]
struct BatteryBank {
    joltage: u64,
}
impl BatteryBank {
    pub fn new(config: BatteryBankConfig) -> Self {
        let joltage = Self::get_joltage(&config.batteries);
        Self { joltage }
    }

    fn get_joltage(batteries: &Vec<u8>) -> u64 {
        let a_max = 0;
        let mut max_joltage = 0_u64;

        let batt_lenth = batteries.len();

        for x in 0..batt_lenth {
            for y in (x + 1)..batt_lenth {
                if x < a_max {
                    break;
                }

                let joltage = batteries[x] * 10 + batteries[y];
                max_joltage = max_joltage.max(joltage as u64);
            }
        }
        max_joltage
    }

    fn get_max_battery_combo(batteries: &[u8], active: usize) -> Vec<u8> {
        if batteries.len() < active || active == 0 {
            return vec![];
        }

        if batteries.len() == active {
            return batteries.to_vec();
        }

        let division = batteries.len() - active;

        let (head_pos, head) = batteries[..=division]
            .iter()
            .enumerate()
            .rev()
            .max_by(|(_, i), (_, j)| i.cmp(j))
            .unwrap();

        let tail = Self::get_max_battery_combo(&batteries[head_pos + 1..], active - 1);

        std::iter::once(head).chain(tail.iter()).copied().collect()
    }
}

struct BatteryBankConfig {
    batteries: Vec<u8>,
}

impl FromStr for BatteryBankConfig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries: Vec<u8> = s
            .trim()
            .chars()
            .map(|c| c.to_string().parse::<u8>().map_err(|e| format!("{e}")))
            .collect::<Result<Vec<u8>, _>>()?;

        Ok(Self { batteries })
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let battery_banks: Vec<BatteryBank> = fs::read_to_string("input")
        .expect("Unable to read input file")
        .trim()
        .lines()
        .map(|l| {
            let config = l
                .trim()
                .parse()
                .expect("Unable to parse battery bank config");
            BatteryBank::new(config)
        })
        .collect();

    let sum: u64 = battery_banks.into_iter().map(|bb| bb.joltage).sum();

    println!("Part 1: {sum}");
}

fn part2() {
    let battery_sets: Vec<Vec<u8>> = fs::read_to_string("input")
        .expect("Unable to read input file")
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .parse::<BatteryBankConfig>()
                .expect("Unable to parse battery bank config")
                .batteries
        })
        .collect();

    let sum: usize = battery_sets
        .into_iter()
        .map(|bb| {
            let r = BatteryBank::get_max_battery_combo(&bb, 12);
            let mut num = 0;
            for a in 0..r.len() {
                num = num * 10 + r[a] as usize;
            }
            num
        })
        .sum();

    println!("Part 2: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "987654321111111
                            811111111111119
                            234234234234278
                            818181911112111";

    #[test]
    fn test_joltage_calc() {
        let battery_banks: Vec<BatteryBank> = TEST_DATA
            .trim()
            .lines()
            .map(|l| {
                let config = l
                    .trim()
                    .parse()
                    .expect("Unable to parse battery bank config");
                BatteryBank::new(config)
            })
            .collect();

        let sum: u64 = battery_banks.into_iter().map(|bb| bb.joltage).sum();

        assert_eq!(sum, 357);
    }

    macro_rules! assert_max_combo {
        ($a: expr, $b: expr, $c: expr) => {
            assert_eq!(BatteryBank::get_max_battery_combo(&$a, $c), $b);
        };
    }

    macro_rules! assert_max_combo_text {
        ($a: expr, $b: expr, $c: expr) => {
            let a: Vec<u8> = $a
                .chars()
                .map(|c| c.to_string().parse::<u8>().expect("Unable to parse"))
                .collect();

            let b: Vec<u8> = $b
                .chars()
                .map(|c| c.to_string().parse::<u8>().expect("Unable to parse"))
                .collect();

            assert_eq!(BatteryBank::get_max_battery_combo(&a, $c), b);
        };
    }

    #[test]
    fn test_max_combo() {
        assert_max_combo!(vec![1], vec![1], 1);
        assert_max_combo!(vec![1, 2], vec![2], 1);
        assert_max_combo!(vec![1, 2, 3], vec![2, 3], 2);
        assert_max_combo!(vec![1, 2, 3], vec![1, 2, 3], 3);
        assert_max_combo!(vec![1, 2, 3], vec![3], 1);

        assert_max_combo_text!("153445", "545", 3);
        assert_max_combo_text!("987654321111111", "987654321111", 12);
        assert_max_combo_text!("811111111111119", "811111111119", 12);
        assert_max_combo_text!("234234234234278", "434234234278", 12);
        assert_max_combo_text!("818181911112111", "888911112111", 12);
    }
}
