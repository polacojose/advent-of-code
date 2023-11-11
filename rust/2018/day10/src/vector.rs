use std::str::FromStr;

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Vector {
    pub pos: (isize, isize),
    pub vel: (isize, isize),
}

impl Vector {
    fn new(pos: (isize, isize), vel: (isize, isize)) -> Self {
        Self { pos, vel }
    }

    pub fn second_delta(&mut self, delta: isize) {
        self.pos.0 += self.vel.0 * delta;
        self.pos.1 += self.vel.1 * delta;
    }
}

#[derive(Debug)]
pub struct UnableToParseVector;
impl FromStr for Vector {
    type Err = UnableToParseVector;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = Regex::new(r#"[-]?\d+"#).unwrap();
        let c = result
            .captures_iter(s)
            .map(|x| x.get(0).unwrap().as_str().parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        Ok(Vector {
            pos: (c[0], c[1]),
            vel: (c[2], c[3]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_vector() {
        let vector = "position=< 9,  1> velocity=< 0,  2>"
            .parse::<Vector>()
            .unwrap();
        assert_eq!(
            vector,
            Vector {
                pos: (9, 1),
                vel: (0, 2)
            }
        );

        let vector = "position=< -9,  1> velocity=< 0,  2>"
            .parse::<Vector>()
            .unwrap();
        assert_eq!(
            vector,
            Vector {
                pos: (-9, 1),
                vel: (0, 2)
            }
        );

        let vector = "position=< -15,  1> velocity=< 0,  2>"
            .parse::<Vector>()
            .unwrap();
        assert_eq!(
            vector,
            Vector {
                pos: (-15, 1),
                vel: (0, 2)
            }
        );
    }

    #[test]
    fn can_calc_position() {
        let mut vector = "position=< 9,  1> velocity=< 1,  2>"
            .parse::<Vector>()
            .unwrap();
        vector.second_delta(0);
        assert_eq!(vector.pos, (9, 1));

        vector.second_delta(1);
        assert_eq!(vector.pos, (10, 3));

        vector.second_delta(1);
        assert_eq!(vector.pos, (11, 5));

        vector.second_delta(2);
        assert_eq!(vector.pos, (13, 9));
    }
}
