use glam::IVec2;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub(super) width: i32,
    pub(super) height: i32,
    pub(super) nodes: Vec<T>,
}

impl<T> Grid<T> {
    pub fn get_nodes(&self) -> &Vec<T> {
        &self.nodes
    }
    pub fn get_by_vector(&self, p: &IVec2) -> Option<&T> {
        Some(&self.nodes[self.vector_to_index(p)?])
    }

    pub fn get_mut_by_vector(&mut self, p: &IVec2) -> Option<&mut T> {
        self.vector_to_index(p).map(|idx| &mut self.nodes[idx])
    }

    pub fn index_to_vector(&self, idx: usize) -> Option<IVec2> {
        if idx >= self.nodes.len() {
            return None;
        }

        let x = idx as i32 % self.width as i32;
        let y = idx as i32 / self.width as i32;

        Some(IVec2 { x, y })
    }

    pub fn vector_to_index(&self, p: &IVec2) -> Option<usize> {
        let ux = p.x;
        let uy = p.y;

        if p.x < 0 || ux >= self.width || p.y < 0 || uy >= self.height {
            return None;
        }
        Some((uy * self.width + ux) as usize)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const EXAMPLE_GRID: &str = r"1234
                                 5678
                                 9876";

    #[test]
    fn test_get_by_vector() {
        let grid = EXAMPLE_GRID.parse::<Grid<char>>().unwrap();
        assert_eq!(grid.get_by_vector(&IVec2 { x: 2, y: 1 }).unwrap(), &'7');
        assert_eq!(grid.get_by_vector(&IVec2 { x: 1, y: 2 }).unwrap(), &'8');
    }

    #[test]
    fn test_get_mut_by_vector() {
        let mut grid = EXAMPLE_GRID.parse::<Grid<char>>().unwrap();

        let x = grid.get_mut_by_vector(&IVec2 { x: 2, y: 1 }).unwrap();
        assert_eq!(x, &'7');

        *x = '8';
        assert_eq!(grid.get_by_vector(&IVec2 { x: 2, y: 1 }).unwrap(), &'8');
    }

    #[test]
    fn test_index_to_vector() {
        let grid = EXAMPLE_GRID.parse::<Grid<char>>().unwrap();

        let v = grid.index_to_vector(7).unwrap();
        assert_eq!(v, IVec2 { x: 3, y: 1 });

        let v = grid.index_to_vector(8).unwrap();
        assert_eq!(v, IVec2 { x: 0, y: 2 });
    }

    #[test]
    fn test_vector_to_index() {
        let grid = EXAMPLE_GRID.parse::<Grid<char>>().unwrap();

        let v = grid.vector_to_index(&IVec2 { x: 3, y: 1 }).unwrap();
        assert_eq!(v, 7);

        let v = grid.vector_to_index(&IVec2 { x: 0, y: 2 }).unwrap();
        assert_eq!(v, 8);
    }
}
