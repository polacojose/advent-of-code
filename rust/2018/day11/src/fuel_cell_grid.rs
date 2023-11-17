use std::sync::{Arc, RwLock, RwLockReadGuard};

const GRID_SIZE: usize = 300;

#[derive(Clone)]
struct FuelCell(i32, i32);

impl FuelCell {
    fn power(&self, grid_serial_number: i32) -> i32 {
        let rack_id = self.0 + 10;
        let mut power_level = rack_id * self.1;
        power_level += grid_serial_number;
        power_level *= rack_id;
        power_level /= 100;
        power_level %= 10;
        power_level -= 5;
        power_level
    }
}

pub struct FuelCellGrid {
    grid_power: Arc<RwLock<[[i32; GRID_SIZE]; GRID_SIZE]>>,
}
impl FuelCellGrid {
    pub fn new(grid_serial_number: i32) -> Self {
        let mut grid_power = [[0; GRID_SIZE]; GRID_SIZE];
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                grid_power[y][x] = FuelCell(x as i32, y as i32).power(grid_serial_number);
            }
        }

        Self {
            grid_power: Arc::new(RwLock::new(grid_power)),
        }
    }

    pub fn highest_power_rect(&self, rect: (u32, u32)) -> (usize, usize) {
        let mut h_p_r = (0, 0);
        let mut max_power = 0;
        for y in 0..=GRID_SIZE - rect.1 as usize {
            for x in 0..=GRID_SIZE - rect.0 as usize {
                let grid = self.grid_power.read().unwrap();
                let total_grid_power = Self::get_grid_power(grid, (x, y), (3, 3));
                if total_grid_power > max_power {
                    max_power = total_grid_power;
                    h_p_r = (x, y);
                }
            }
        }

        return h_p_r;
    }

    pub fn highest_power_rect_flex_rect(&self) -> (usize, usize, usize) {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let mut handles = Vec::new();

        for size in 1..GRID_SIZE {
            let grid_arc = self.grid_power.clone();
            handles.push(rt.spawn(async move {
                let mut h_p_r = (0, 0);
                let mut max_power = 0;
                let mut max_power_size = 0;
                for y in 0..=GRID_SIZE - size {
                    for x in 0..=GRID_SIZE - size {
                        let grid = grid_arc.read().unwrap();
                        let total_grid_power = Self::get_grid_power(grid, (x, y), (size, size));
                        if total_grid_power > max_power {
                            max_power = total_grid_power;
                            h_p_r = (x, y);
                            max_power_size = size;
                        }
                    }
                }
                return (h_p_r, max_power, max_power_size);
            }));
        }

        let result = rt.block_on(async {
            let mut future_iter = futures::future::join_all(handles).await.into_iter();
            let mut h_p_r = (0, 0);
            let mut max_power = 0;
            let mut max_power_size = 0;

            while let Some(Ok((hpr, mp, mps))) = future_iter.next() {
                if mp > max_power {
                    max_power = mp;
                    h_p_r = hpr;
                    max_power_size = mps;
                }
            }
            return (h_p_r.0, h_p_r.1, max_power_size);
        });

        return result;
    }

    fn get_grid_power(
        grid: RwLockReadGuard<'_, [[i32; 300]; 300]>,
        tl: (usize, usize),
        size: (usize, usize),
    ) -> i32 {
        let mut power = 0;
        for y in tl.1..(tl.1 + size.1) {
            for x in tl.0..(tl.0 + size.0) {
                power += grid[y][x] as i32;
            }
        }
        return power;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_power() {
        assert_eq!(FuelCell(3, 5).power(8), 4);
        assert_eq!(FuelCell(122, 79).power(57), -5);
        assert_eq!(FuelCell(217, 196).power(39), 0);
        assert_eq!(FuelCell(101, 153).power(71), 4);
    }

    #[test]
    fn can_calculate_highest_power_rect() {
        assert_eq!(FuelCellGrid::new(18).highest_power_rect((3, 3)), (33, 45));
        assert_eq!(FuelCellGrid::new(42).highest_power_rect((3, 3)), (21, 61));
    }

    #[test]
    fn can_calculate_highest_power_flex_rect() {
        assert_eq!(
            FuelCellGrid::new(18).highest_power_rect_flex_rect(),
            (90, 269, 16)
        );
    }
}
