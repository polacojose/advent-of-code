use crate::{Nodable, node::ReverseOrderedNode};

pub trait AStarHelper<T>
where
    T: Nodable,
{
    //Heuristic Function
    fn hf(&self, node: &T) -> u64;

    //Distance Function
    fn d(&self, a: &T, b: &T) -> u64;

    fn neighbors(&self, node: &T) -> Vec<ReverseOrderedNode<T>>;
}
