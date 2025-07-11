use std::str::FromStr;

use crate::nodes::node::Node;

#[derive(Debug)]
pub struct NodeGrid {
    nodes: Vec<Node>,
    width: usize,
}

#[derive(Debug)]
pub struct UnableToParse;

impl FromStr for NodeGrid {
    type Err = UnableToParse;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: Vec<Node> = {
            s.lines()
                .enumerate()
                .map(|(i, s)| {
                    let mut n: Node = s.parse().unwrap();
                    n.id = i;
                    n
                })
                .collect()
        };

        let goal_node_id = Self::goal_node(&nodes).ok_or(UnableToParse)?.id;

        let width = nodes
            .iter()
            .max_by_key(|n| n.position.x)
            .ok_or(UnableToParse)?
            .position
            .x
            + 1;

        if let Some(goal_node) = nodes.get_mut(goal_node_id) {
            goal_node.goal_data = true;
            Ok(NodeGrid { width, nodes })
        } else {
            Err(UnableToParse)
        }
    }
}

impl NodeGrid {
    pub fn viable_pairs(&self) -> Vec<(&Node, &Node)> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, a)| a.used > 0)
            .flat_map(|(a_i, a)| {
                self.nodes
                    .iter()
                    .enumerate()
                    .filter(|(b_i, b)| &a_i != b_i && a.used <= b.avail())
                    .map(|(_, b)| (a, b))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    pub fn goal_node(nodes: &[Node]) -> Option<&Node> {
        nodes
            .iter()
            .filter(|n| n.position.y == 0)
            .max_by_key(|n| n.position.x)
    }

    /// Returns the IDs of the nodes that can be transfered into from the given node
    pub fn viable_transfer_to_nodes(&self, node: &Node) -> Vec<usize> {
        [[-1, 0], [0, 1], [1, 0], [0, -1]]
            .into_iter()
            .filter_map(|[x, y]| {
                let n =
                    self.get_node_at(node.position.x as isize + x, node.position.y as isize + y)?;
                if n.avail() >= node.used {
                    Some(n.id)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns the IDs of the nodes that can be transfered from, into the given node
    pub fn viable_transfer_from_nodes(&self, node: &Node) -> Vec<usize> {
        [[-1, 0], [0, 1], [1, 0], [0, -1]]
            .into_iter()
            .filter_map(|[x, y]| {
                let n =
                    self.get_node_at(node.position.x as isize + x, node.position.y as isize + y)?;
                if n.used <= node.avail() {
                    Some(n.id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_node_at(&self, x: isize, y: isize) -> Option<&Node> {
        if x < 0 || y < 0 {
            return None;
        }

        let (x, y) = (x as usize, y as usize);

        self.nodes.get(y * self.width + x)
    }
}

pub struct UnableToEmptyNode;
impl NodeGrid {
    pub fn empty_node(node: &Node) -> Result<(), UnableToEmptyNode> {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    macro_rules! assert_viable_transfer_to_len {
        ($node_grid:expr, $node_index:expr, $length:expr) => {
            assert_eq!(
                $node_grid
                    .viable_transfer_to_nodes(&$node_grid.nodes[$node_index])
                    .len(),
                $length
            );
        };
    }

    macro_rules! assert_viable_transfer_from_len {
        ($node_grid:expr, $node_index:expr, $length:expr) => {
            assert_eq!(
                $node_grid
                    .viable_transfer_from_nodes(&$node_grid.nodes[$node_index])
                    .len(),
                $length
            );
        };
    }

    const TEST_STR: &str = r"
    /dev/grid/node-x0-y0   10T    8T     2T   80%
    /dev/grid/node-x0-y1   11T    6T     5T   54%
    /dev/grid/node-x0-y2   32T   28T     4T   87%
    /dev/grid/node-x1-y0    9T    7T     2T   77%
    /dev/grid/node-x1-y1    8T    0T     8T    0%
    /dev/grid/node-x1-y2   11T    7T     4T   63%
    /dev/grid/node-x2-y0   10T    6T     4T   60%
    /dev/grid/node-x2-y1    9T    8T     1T   88%
    /dev/grid/node-x2-y2    9T    6T     3T   66%";

    #[test]
    fn locates_goal_data_node() {
        let node_grid: NodeGrid = TEST_STR.trim().parse().unwrap();

        if let Some(goal_node) = NodeGrid::goal_node(&node_grid.nodes) {
            assert!(
                goal_node.position.y == 0,
                "Destination node not at top position!"
            );
            assert!(
                goal_node.position.x == 2,
                "Destination node not at right position!"
            );
        } else {
            assert!(false, "Destination node not located!")
        }
    }

    #[test]
    fn gets_viable_transfer_to_nodes() {
        let node_grid: NodeGrid = TEST_STR.trim().parse().unwrap();

        assert_viable_transfer_to_len!(node_grid, 0, 0);
        assert_viable_transfer_to_len!(node_grid, 1, 1);
        assert_viable_transfer_to_len!(node_grid, 2, 0);
        assert_viable_transfer_to_len!(node_grid, 3, 1);
        assert_viable_transfer_to_len!(node_grid, 4, 4);
        assert_viable_transfer_to_len!(node_grid, 5, 1);
        assert_viable_transfer_to_len!(node_grid, 6, 0);
        assert_viable_transfer_to_len!(node_grid, 7, 1);
        assert_viable_transfer_to_len!(node_grid, 8, 0);
    }

    #[test]
    fn gets_viable_transfer_from_nodes() {
        let node_grid: NodeGrid = TEST_STR.trim().parse().unwrap();

        assert_viable_transfer_from_len!(node_grid, 0, 0);
        assert_viable_transfer_from_len!(node_grid, 1, 1);
        assert_viable_transfer_from_len!(node_grid, 2, 0);
        assert_viable_transfer_from_len!(node_grid, 3, 1);
        assert_viable_transfer_from_len!(node_grid, 4, 4);
        assert_viable_transfer_from_len!(node_grid, 5, 1);
        assert_viable_transfer_from_len!(node_grid, 6, 0);
        assert_viable_transfer_from_len!(node_grid, 7, 1);
        assert_viable_transfer_from_len!(node_grid, 8, 0);
    }
}
