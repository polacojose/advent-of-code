use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IVec2 {
    pub x: isize,
    pub y: isize,
}

impl Add for IVec2 {
    type Output = IVec2;

    fn add(self, rhs: Self) -> Self::Output {
        IVec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: Self) -> Self::Output {
        IVec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl IVec2 {
    pub fn abs_diff(self, rhs: Self) -> Self {
        IVec2 {
            x: self.x.abs_diff(rhs.x) as isize,
            y: self.y.abs_diff(rhs.y) as isize,
        }
    }
}

impl IVec2 {
    pub fn rotate_90(&self) -> IVec2 {
        IVec2 {
            x: -self.y,
            y: self.x,
        }
    }
}
