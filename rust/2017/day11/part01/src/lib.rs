use std::{ops::Sub, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub struct HexCoord {
    pub q: isize,
    pub r: isize,
    pub s: isize,
}

pub enum Dir {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

#[derive(Debug)]
pub struct UnableToDebug;
impl FromStr for Dir {
    type Err = UnableToDebug;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(Dir::N),
            "ne" => Ok(Dir::NE),
            "se" => Ok(Dir::SE),
            "s" => Ok(Dir::S),
            "sw" => Ok(Dir::SW),
            "nw" => Ok(Dir::NW),
            _ => Err(UnableToDebug),
        }
    }
}

impl Sub for HexCoord {
    type Output = HexCoord;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
            s: self.s - rhs.s,
        }
    }
}

impl HexCoord {
    pub fn new(q: isize, r: isize, s: isize) -> Self {
        Self { q, r, s }
    }

    pub fn distance(&self, rhs: HexCoord) -> usize {
        let difference = *self - rhs;
        (difference.q.abs() + difference.r.abs() + difference.s.abs()) as usize / 2
    }

    pub fn shift(&mut self, dir: &Dir) {
        match dir {
            Dir::N => {
                self.s += 1;
                self.r -= 1;
            }
            Dir::NE => {
                self.q += 1;
                self.r -= 1;
            }
            Dir::SE => {
                self.q += 1;
                self.s -= 1;
            }
            Dir::S => {
                self.s -= 1;
                self.r += 1;
            }
            Dir::SW => {
                self.q -= 1;
                self.r += 1;
            }
            Dir::NW => {
                self.q -= 1;
                self.s += 1;
            }
        }
    }
}
