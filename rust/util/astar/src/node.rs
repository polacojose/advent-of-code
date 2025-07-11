use std::ops::Add;

use crate::Nodable;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ReverseOrderedNode<T>
where
    T: Nodable,
{
    pub node: T,
    pub cost: u64,
}

impl<T> Add<u64> for ReverseOrderedNode<T>
where
    T: Nodable,
{
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self {
            node: self.node,
            cost: self.cost.saturating_add(rhs),
        }
    }
}

impl<T> PartialOrd for ReverseOrderedNode<T>
where
    T: Nodable,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl<T> Ord for ReverseOrderedNode<T>
where
    T: Nodable,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
