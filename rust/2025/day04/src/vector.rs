use std::ops::Add;

#[derive(Debug, Clone, Copy)]
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
