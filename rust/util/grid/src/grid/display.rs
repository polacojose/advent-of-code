use std::fmt::Display;

use crate::grid::map::Grid;

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.nodes
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let i = i + 1;
                if i % self.width as usize == 0 {
                    writeln!(f, "{}", n)
                } else {
                    write!(f, "{}", n)
                }
            })
            .collect::<Result<(), _>>()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const TEST_MAP_STR: &str = r"..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
";

    #[test]
    pub fn test_map_display() {
        let map = TEST_MAP_STR.parse::<Grid<String>>().unwrap();
        assert_eq!(format!("{}", map), TEST_MAP_STR);
    }
}
