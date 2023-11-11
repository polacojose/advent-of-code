use crate::vector::Vector;

pub struct StarGrid {
    resolution: usize,
    vectors: Vec<Vector>,
}

impl StarGrid {
    pub fn new(resolution: usize, vectors: Vec<Vector>) -> Self {
        Self {
            vectors,
            resolution,
        }
    }

    pub fn delta(&mut self, delta: isize) {
        self.vectors.iter_mut().for_each(|v| v.second_delta(delta));
    }
}

impl StarGrid {
    fn rect(vectors: &Vec<Vector>) -> Rect {
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;
        let mut max_x = isize::MIN;
        let mut max_y = isize::MIN;

        for v in vectors {
            min_x = min_x.min(v.pos.0);
            min_y = min_y.min(v.pos.1);
            max_x = max_x.max(v.pos.0);
            max_y = max_y.max(v.pos.1);
        }

        Rect {
            origin: (min_x, min_y),
            width: max_x.abs_diff(min_x),
            height: max_y.abs_diff(min_y),
        }
    }

    fn empty_grid(&self) -> Vec<Vec<bool>> {
        let mut vec_grid = Vec::new();
        for _ in 0..=self.resolution {
            let mut empty_row = Vec::new();
            for _ in 0..=self.resolution {
                empty_row.push(false);
            }
            vec_grid.push(empty_row.clone());
        }
        return vec_grid;
    }

    pub fn x_divider(&self) -> f64 {
        let rect = Self::rect(&self.vectors);

        (1.0 as f64).max(rect.width as f64 / self.resolution as f64)
    }

    pub fn output(&self) -> String {
        let rect = Self::rect(&self.vectors);

        let x_offset = 0 - rect.origin.0;
        let y_offset = 0 - rect.origin.1;

        let x_divider = (1.0 as f64).max(rect.width as f64 / self.resolution as f64);
        let y_divider = (1.0 as f64).max(rect.height as f64 / self.resolution as f64);

        println!("{:?} {:?}", x_divider, y_divider);

        let mut g = self.empty_grid();
        for v in &self.vectors {
            let divided_pos = (
                (x_offset + v.pos.0) as f64 / x_divider,
                (y_offset + v.pos.1) as f64 / y_divider,
            );

            g[divided_pos.1 as usize][divided_pos.0 as usize] = true;
        }

        let mut output = Vec::new();
        for row in g {
            let mut output_row = Vec::new();
            for cell in row {
                output_row.push(cell);
            }
            output.push(
                output_row
                    .into_iter()
                    .map(|x| match x {
                        true => '#',
                        false => '.',
                    })
                    .collect::<String>(),
            );
        }

        return output.join("\n");
    }
}

#[derive(Debug)]
struct Rect {
    origin: (isize, isize),
    width: usize,
    height: usize,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn can_print_grid() {
        let expected = r#"
........#..............
................#......
.........#.#..#........
.......................
#..........#.#.......#.
...............#.......
....#..................
..#.#....#.............
.......#...............
......#................
...#...#.#...#.........
....#..#..#.........#..
.......#...............
...........#..#........
#...........#..........
...#.......#...........
.......................
.......................
.......................
.......................
.......................
.......................
......................."#
            .trim();

        let vectors = fs::read_to_string("test-input.txt")
            .unwrap()
            .lines()
            .map(|x| x.parse::<Vector>().unwrap())
            .collect();
        let grid = StarGrid::new(22, vectors);
        println!("{}", grid.output());
        assert_eq!(expected, grid.output());
    }

    #[test]
    fn can_print_with_delta() {
        let expected = r#"
#...#..###.
#...#...#..
#...#...#..
#####...#..
#...#...#..
#...#...#..
#...#...#..
#...#..###.
...........
...........
..........."#
            .trim();

        let vectors = fs::read_to_string("test-input.txt")
            .unwrap()
            .lines()
            .map(|x| x.parse::<Vector>().unwrap())
            .collect();
        let mut grid = StarGrid::new(10, vectors);

        grid.delta(3);
        println!("{}", grid.output());
        assert_eq!(expected, grid.output());
    }
}
