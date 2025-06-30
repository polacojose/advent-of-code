use std::collections::HashSet;

use crate::vector::IVec2;

#[derive(Clone)]
pub(super) enum MapNodeKind {
    Empty,
    GuardUp,
    GuardDown,
    GuardLeft,
    GuardRight,
    Path,
    Barrier,
}

impl MapNodeKind {
    pub fn is_guard(&self) -> bool {
        match self {
            MapNodeKind::GuardUp
            | MapNodeKind::GuardDown
            | MapNodeKind::GuardLeft
            | MapNodeKind::GuardRight => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Map {
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) nodes: Vec<MapNodeKind>,
    pub(super) guard_move_vector: IVec2,
}

#[derive(Debug)]
pub enum SolveCompletion {
    OffTheMap,
    InALoop,
}

#[derive(Debug)]
pub struct SolveError(&'static str);
impl Map {
    pub fn num_walked_positions(&mut self) -> usize {
        let _ = self.solve();
        self.nodes
            .iter()
            .filter(|n| matches!(n, MapNodeKind::Path))
            .count()
    }

    pub fn solve(&mut self) -> Result<SolveCompletion, SolveError> {
        let mut loop_detect_set = HashSet::new();

        let mut guard_pos = self
            .index_to_vector(
                self.nodes
                    .iter()
                    .enumerate()
                    .find_map(|(idx, n)| if n.is_guard() { Some(idx) } else { None })
                    .ok_or(SolveError("No guard present."))?,
            )
            .ok_or(SolveError("Incorrect index_to_vector conversion"))?;

        //While the guard is on the map
        loop {
            //Loop Detection
            {
                let e = (guard_pos, self.guard_move_vector);
                if loop_detect_set.contains(&e) {
                    return Ok(SolveCompletion::InALoop);
                }
                loop_detect_set.insert(e);
            }

            let fn_idx = self.forward_to_next_available_node(guard_pos);

            if let None = fn_idx {
                return Ok(SolveCompletion::OffTheMap);
            }

            let forward_node_idx = fn_idx.ok_or(SolveError("No foward node"))?;

            guard_pos = self
                .index_to_vector(forward_node_idx)
                .ok_or(SolveError("No guard node"))?;

            self.rotate_guard_at(forward_node_idx)
                .map_err(|_| SolveError("Unable to rotate guard"))?;
        }
    }
}

impl Map {
    fn forward_to_next_available_node(&mut self, guard_pos: IVec2) -> Option<usize> {
        let last_guard_kind = self.nodes[self.vector_to_index(guard_pos)?].clone();

        let mut last_idx = None;
        let mut next_vec = guard_pos;

        loop {
            let idx = self.vector_to_index(next_vec);

            if let Some(idx) = idx {
                let node_kind = &self.nodes[idx];
                match node_kind {
                    MapNodeKind::Barrier => break,
                    _ => {
                        self.nodes[idx] = MapNodeKind::Path;
                        last_idx = Some(idx)
                    }
                }
            } else {
                return None;
            }

            next_vec = next_vec + self.guard_move_vector;
        }

        if let Some(idx) = last_idx {
            self.nodes[idx] = last_guard_kind;
        }
        last_idx
    }

    #[cfg(test)]
    fn rotate_guard(&mut self) -> Result<(), ()> {
        let idx = self
            .nodes
            .iter()
            .enumerate()
            .find_map(|(i, n)| if n.is_guard() { Some(i) } else { None })
            .ok_or(())?;
        self.rotate_guard_at(idx)
    }

    fn rotate_guard_at(&mut self, idx: usize) -> Result<(), ()> {
        let guard = self.nodes.get_mut(idx).ok_or(())?;

        let new_guard_kind = match guard {
            MapNodeKind::GuardUp => Ok(MapNodeKind::GuardRight),
            MapNodeKind::GuardRight => Ok(MapNodeKind::GuardDown),
            MapNodeKind::GuardDown => Ok(MapNodeKind::GuardLeft),
            MapNodeKind::GuardLeft => Ok(MapNodeKind::GuardUp),
            _ => Err(()),
        }?;

        self.guard_move_vector = self.guard_move_vector.rotate_90();
        *guard = new_guard_kind;
        Ok(())
    }

    fn index_to_vector(&self, idx: usize) -> Option<IVec2> {
        if idx >= self.nodes.len() {
            return None;
        }

        let x = (idx % self.width) as isize;
        let y = (idx / self.width) as isize;

        Some(IVec2 { x, y })
    }

    fn vector_to_index(&self, p: IVec2) -> Option<usize> {
        let ux = p.x as usize;
        let uy = p.y as usize;

        if p.x < 0 || ux >= self.width || p.y < 0 || uy >= self.height {
            return None;
        }
        Some((uy * self.width + ux) as usize)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const TEST_MAP_STR: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    const TEST_MAP_SOLVED_STR: &str = r"....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..
";

    #[test]
    fn test_rotate_guard() {
        let mut map: Map = "^".parse().unwrap();
        map.rotate_guard().unwrap();
        assert_eq!(format!("{}", map), ">\n");
        map.rotate_guard().unwrap();
        assert_eq!(format!("{}", map), "v\n");
        map.rotate_guard().unwrap();
        assert_eq!(format!("{}", map), "<\n");
        map.rotate_guard().unwrap();
        assert_eq!(format!("{}", map), "^\n");
    }

    #[test]
    fn test_solve() {
        let mut map: Map = TEST_MAP_STR.parse().unwrap();
        let _ = map.solve();
        assert_eq!(format!("{map}"), TEST_MAP_SOLVED_STR);

        assert_eq!(map.num_walked_positions(), 41);
    }
}
