use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    hash::Hash,
};

use crate::{helper::AStarHelper, node::ReverseOrderedNode};

pub mod helper;
pub mod node;

pub trait Nodable: Clone + Hash + Eq + Debug {}

#[derive(Debug, PartialEq, Eq)]
pub struct UnableToFindPath;

pub struct AStar {}
impl AStar {
    pub fn find_path_length<T, AS>(start: &T, end: &T, helper: AS) -> Result<u64, UnableToFindPath>
    where
        T: Nodable,
        AS: AStarHelper<T>,
    {
        // The set of discovered nodes that may need to be (re-)expanded.
        // Initially, only the start node is known.
        // This is implemented as a min-heap.
        let mut open_set: BinaryHeap<ReverseOrderedNode<T>> = BinaryHeap::default();
        open_set.push(ReverseOrderedNode {
            node: start.clone(),
            cost: 0,
        });

        // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from the start
        // to n currently known.
        let mut came_from: HashMap<T, T> = Default::default();

        // For node n, gScore[n] is the currently known cost of the cheapest path from start to n.
        let mut g_score: HashMap<T, u64> = Default::default();
        g_score.insert(start.clone(), 0);

        let mut n = 0;
        while let Some(current) = open_set.pop() {
            n += 1;

            println!("n:{n} current:{:?}", current.node);

            let current_g_score = *g_score.get(&current.node).unwrap();

            //The node with the lowest cost was found.
            if current.node == *end {
                return Ok(current_g_score);
            }

            for neighbor in helper.neighbors(&current.node) {
                let tentative_g_score = current_g_score + neighbor.cost;
                if tentative_g_score
                    < g_score
                        .get(&neighbor.node)
                        .unwrap_or(&u64::MAX)
                        .saturating_add(helper.d(&current.node, &neighbor.node))
                {
                    came_from.insert(neighbor.node.clone(), current.node.clone());
                    g_score.insert(neighbor.node.clone(), tentative_g_score);
                    let h = helper.hf(&neighbor.node);
                    //println!("Inserting: {:#?}", neighbor);
                    open_set.push(neighbor + current_g_score + h);
                }
            }
        }

        Err(UnableToFindPath)
    }
}

#[cfg(test)]
mod tests {
    use glam::I64Vec2;

    use super::*;

    const A_IVEC: I64Vec2 = I64Vec2 { x: 94, y: 34 };
    const B_IVEC: I64Vec2 = I64Vec2 { x: 22, y: 67 };

    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    enum NodeKind {
        A(I64Vec2),
        B(I64Vec2),
        None,
    }

    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    struct Node {
        pos: I64Vec2,
        kind: NodeKind,
    }

    impl Nodable for Node {}

    struct TestHelper {
        path_a: I64Vec2,
        path_b: I64Vec2,
        dest: I64Vec2,
    }
    impl AStarHelper<Node> for TestHelper {
        fn hf(&self, node: &Node) -> u64 {
            self.d(
                node,
                &Node {
                    pos: self.dest,
                    kind: NodeKind::None,
                },
            )
        }

        fn d(&self, a: &Node, b: &Node) -> u64 {
            let (tokens, ivec) = match a.kind {
                NodeKind::A(i64_vec2) => (3, i64_vec2),
                NodeKind::B(i64_vec2) => (1, i64_vec2),
                _ => panic!(""),
            };

            (((b.pos - a.pos) / ivec).length_squared() * tokens) as u64
        }

        fn neighbors(&self, node: &Node) -> Vec<ReverseOrderedNode<Node>> {
            let mut v = Vec::new();

            let a_n = self.path_a + node.pos;
            let next_a = ReverseOrderedNode {
                node: Node {
                    pos: a_n,
                    kind: NodeKind::A(A_IVEC),
                },
                cost: 3,
            };

            if a_n.x <= self.dest.x && a_n.y <= self.dest.y {
                v.push(next_a);
            }

            let b_n = self.path_b + node.pos;
            let next_b = ReverseOrderedNode {
                node: Node {
                    pos: b_n,
                    kind: NodeKind::B(B_IVEC),
                },
                cost: 1,
            };

            if b_n.x <= self.dest.x && b_n.y <= self.dest.y {
                v.push(next_b);
            }

            v
        }
    }

    #[test]
    fn test_path_finding() {
        let start = Node {
            pos: I64Vec2 { x: 0, y: 0 },
            kind: NodeKind::A(A_IVEC),
        };
        let end = Node {
            pos: I64Vec2 { x: 8400, y: 5400 },
            kind: NodeKind::A(A_IVEC),
        };

        let cost = AStar::find_path_length(
            &start,
            &end,
            TestHelper {
                path_a: I64Vec2 { x: 94, y: 34 },
                path_b: I64Vec2 { x: 22, y: 67 },
                dest: end.pos,
            },
        );
        println!("{:?}", cost);

        assert_eq!(cost, Ok(280));
    }
}
