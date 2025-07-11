use std::collections::HashMap;

use glam::I64Vec2;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod parse;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Robot {
    pos: I64Vec2,
    vel: I64Vec2,
}

impl Robot {
    pub fn get_quad(&self, space: I64Vec2) -> Option<u64> {
        get_quad(self.pos, space)
    }
}

fn is_middle(a: i64, b: i64) -> bool {
    let mid = b / 2;

    //Get middle
    if b % 2 == 0 {
        a == mid || a + 1 == mid
    } else {
        a == mid
    }
}

pub fn get_quad(pos: I64Vec2, space: I64Vec2) -> Option<u64> {
    if is_middle(pos.x, space.x) || is_middle(pos.y, space.y) {
        return None;
    }

    let placement = pos * 2 / space;

    let quadrant_index = placement.y * 2 + placement.x;

    Some(quadrant_index as u64)
}

pub struct RobotPlotter {
    robots: Vec<Robot>,
}

impl RobotPlotter {
    pub fn positions_at_steps(&self, steps: u64, width: u64, height: u64) -> Vec<I64Vec2> {
        self.robots
            .iter()
            .map(|r| {
                (r.vel * steps as i64 + r.pos).rem_euclid(I64Vec2 {
                    x: width as i64,
                    y: height as i64,
                })
            })
            .collect()
    }

    pub fn safety_at_steps(&self, steps: u64, width: u64, height: u64) -> u64 {
        let quad_map = self
            .positions_at_steps(steps, width, height)
            .into_iter()
            .filter_map(|p| {
                get_quad(
                    p,
                    I64Vec2 {
                        x: width as i64,
                        y: height as i64,
                    },
                )
            })
            .fold(HashMap::new(), |mut acc, a| {
                acc.entry(a).and_modify(|x| *x += 1).or_insert(1);
                acc
            });

        quad_map.into_iter().fold(1, |acc, (_, v2)| acc * v2)
    }

    pub fn display_at_steps(&self, steps: u64, width: u64, height: u64) -> String {
        let positions = self
            .positions_at_steps(steps, width, height)
            .into_iter()
            .fold(vec![0; (width * height) as usize], |mut acc, v| {
                acc[(v.y * width as i64 + v.x) as usize] += 1;
                acc
            });

        let mut output = String::with_capacity((width * height) as usize);
        for y in 0..height as i64 {
            for x in 0..width as i64 {
                let a = positions[(y * width as i64 + x) as usize];
                if a > 0 {
                    output += " ";
                    output += &a.to_string();
                } else {
                    output += " .";
                }
            }
            output += "\n";
        }

        output
    }

    pub fn get_most_symetrical_at_steps(
        &self,
        max_steps: u64,
        width: u64,
        height: u64,
    ) -> (u64, String) {
        let (_, best_steps, best_grid) = (0..max_steps)
            .into_par_iter()
            .fold(
                || (f64::MAX, 0, vec![0; (width * height) as usize]),
                |last @ (bsc, _, _), steps| {
                    let mut grid = vec![0; (width * height) as usize];

                    let positions = self.positions_at_steps(steps, width, height);
                    for v in positions {
                        grid[(v.y * width as i64 + v.x) as usize] += 1;
                    }

                    let score = wald_wolfowitz_test(
                        grid.iter()
                            .map(|x| if *x > 0 { 1 } else { 0 })
                            .collect::<Vec<_>>()
                            .as_slice(),
                    );

                    if score < bsc {
                        (score, steps, grid)
                    } else {
                        last
                    }
                },
            )
            .reduce(
                || (f64::MAX, 0, vec![0; (width * height) as usize]),
                |a @ (bsc_a, s_a, _), b @ (bsc_b, s_b, _)| {
                    if bsc_a == bsc_b {
                        if s_a < s_b { a } else { b }
                    } else {
                        if bsc_a < bsc_b { a } else { b }
                    }
                },
            );

        let mut output = String::with_capacity((width * height) as usize);
        for y in 0..height as i64 {
            for x in 0..width as i64 {
                let a = best_grid[(y * width as i64 + x) as usize];
                if a > 0 {
                    output += " ";
                    output += &a.to_string();
                } else {
                    output += " .";
                }
            }
            output += "\n";
        }
        return (best_steps, output);
    }
}

fn wald_wolfowitz_test(data: &[u8]) -> f64 {
    assert!(
        data.iter().all(|&x| x == 0 || x == 1),
        "Only binary values allowed"
    );

    let mut runs = 1;
    for i in 1..data.len() {
        if data[i] != data[i - 1] {
            runs += 1;
        }
    }

    let n1 = data.iter().filter(|&&x| x == 1).count() as f64;
    let n2 = data.iter().filter(|&&x| x == 0).count() as f64;

    if n1 == 0.0 || n2 == 0.0 {
        return 0.0; // No test possible if only one class
    }

    let expected_runs = 1.0 + (2.0 * n1 * n2) / (n1 + n2);
    let variance_runs =
        (2.0 * n1 * n2 * (2.0 * n1 * n2 - n1 - n2)) / (((n1 + n2).powi(2)) * (n1 + n2 - 1.0));

    let z = (runs as f64 - expected_runs) / variance_runs.sqrt();

    z
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_position() {
        let rp = RobotPlotter {
            robots: vec![Robot {
                pos: I64Vec2 { x: 2, y: 4 },
                vel: I64Vec2 { x: 2, y: -3 },
            }],
        };

        assert_eq!(
            rp.positions_at_steps(5, 11, 7),
            vec![I64Vec2 { x: 1, y: 3 }]
        );
    }

    macro_rules! test_quad_10 {
        ($x:expr, $y:expr, $r:expr ) => {
            let quad = Robot {
                pos: I64Vec2 { x: $x, y: $y },
                vel: I64Vec2::default(),
            }
            .get_quad(I64Vec2 { x: 10, y: 10 });
            assert_eq!(quad, $r);
        };
    }

    #[test]
    fn test_robot_quad() {
        test_quad_10!(0, 0, Some(0));
        test_quad_10!(7, 0, Some(1));
        test_quad_10!(0, 7, Some(2));
        test_quad_10!(0, 4, None);
        test_quad_10!(4, 0, None);
        test_quad_10!(0, 5, None);
        test_quad_10!(5, 0, None);
        test_quad_10!(9, 9, Some(3));
    }

    #[test]
    fn test_safety() {
        let rp = "p=0,4 v=3,-3
                  p=6,3 v=-1,-3
                  p=10,3 v=-1,2
                  p=2,0 v=2,-1
                  p=0,0 v=1,3
                  p=3,0 v=-2,-2
                  p=7,6 v=-1,-3
                  p=3,0 v=-1,-2
                  p=9,3 v=2,3
                  p=7,3 v=-1,2
                  p=2,4 v=2,-3
                  p=9,5 v=-3,-3"
            .parse::<RobotPlotter>()
            .unwrap();

        let safety = rp.safety_at_steps(100, 11, 7);
        assert_eq!(safety, 12);
    }
}
