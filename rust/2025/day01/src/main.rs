use std::{error::Error, fs, str::FromStr};

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "L" => Self::Left,
            "R" => Self::Right,
            t => return Err(format!("Unknown type: {t}")),
        })
    }
}

#[derive(Debug)]
struct ZeroSafeCounter {
    pos: i32,
    size: i32,
    zeros: i32,
    count_passthrough: bool,
}

impl ZeroSafeCounter {
    pub fn rotate(mut self, rotation: &Rotation) -> Self {
        assert!(self.pos >= 0);
        assert!(self.pos < self.size);

        if self.count_passthrough {
            let rotation_left = {
                self.zeros += (rotation.amount / self.size).abs();
                rotation.amount % self.size
            };
            let new_pos = self.pos + rotation_left;
            if (self.pos != 0 && new_pos <= 0) || new_pos >= self.size {
                self.zeros += 1;
            }
            self.pos = new_pos.rem_euclid(self.size);
        } else {
            self.pos = {
                let n = self.pos + rotation.amount;
                n.rem_euclid(self.size)
            };

            if self.pos == 0 {
                self.zeros += 1;
            }
        }

        self
    }
}

#[derive(Debug)]
struct Rotation {
    amount: i32,
}

impl FromStr for Rotation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, amount_str) = s.split_at_checked(1).ok_or("")?;

        let dir: Dir = dir_str.parse()?;
        let amount: i32 = amount_str.parse()?;

        Ok(Self {
            amount: match dir {
                Dir::Left => -amount,
                Dir::Right => amount,
            },
        })
    }
}

fn main() {
    let safe = count_zeros(false);
    println!("Part 1: {}", safe.zeros);

    let safe = count_zeros(true);
    println!("Part 2: {}", safe.zeros);
}

fn count_zeros(count_passthrough: bool) -> ZeroSafeCounter {
    let rotations: Vec<Rotation> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let safe = rotations.into_iter().fold(
        ZeroSafeCounter {
            pos: 50,
            size: 100,
            zeros: 0,
            count_passthrough,
        },
        |safe, rot| safe.rotate(&rot),
    );
    safe
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_DATA: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    macro_rules! test_zeros {
        ($start: expr, $passthrough: expr, $rots: expr, $result: expr) => {
            let safe = $rots.iter().fold(
                ZeroSafeCounter {
                    pos: $start,
                    size: 100,
                    zeros: 0,
                    count_passthrough: $passthrough,
                },
                |safe, rot| safe.rotate(rot),
            );

            assert_eq!(safe.zeros, $result);
        };
    }

    #[test]
    fn test_zeros() {
        let rotations: Vec<Rotation> = TEST_DATA
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        test_zeros!(50, false, rotations, 3);
    }

    #[test]
    fn test_zeros_passthrough() {
        let rotations: Vec<Rotation> = vec![Rotation { amount: 1000 }];
        test_zeros!(50, true, rotations, 10);
        test_zeros!(1, true, vec![Rotation { amount: -1 }], 1);
        test_zeros!(1, true, vec![Rotation { amount: -2 }], 1);
        test_zeros!(99, true, vec![Rotation { amount: 1 }], 1);
        test_zeros!(99, true, vec![Rotation { amount: 2 }], 1);
    }

    #[test]
    fn test_zeros_passthrough2() {
        let rotations: Vec<Rotation> = TEST_DATA
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        test_zeros!(50, true, rotations, 6);
    }
}
