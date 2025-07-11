use std::{error::Error, str::FromStr};

use nalgebra::{DMatrix, DVector};
use regex::Regex;

#[derive(Debug)]
pub struct ClawSolver {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    px: f64,
    py: f64,
}

impl ClawSolver {
    pub fn solve(&self, offset: f64) -> Option<Vec<(u64, u64)>> {
        let a = DMatrix::from_row_slice(2, 2, &[self.ax, self.bx, self.ay, self.by]);
        let b = DVector::from_vec(vec![self.px + offset, self.py + offset]);

        a.lu().solve(&b).map(|x| {
            x.into_iter()
                .collect::<Vec<_>>()
                .windows(2)
                .filter_map(|x| {
                    if *x[0] < 0.0
                        || *x[1] < 0.0
                        || (x[0].round() - x[0]).abs() > 0.001
                            && (x[1].round() - x[1]).abs() > 0.001
                    {
                        None
                    } else {
                        Some((x[0].round() as u64, x[1].round() as u64))
                    }
                })
                .collect()
        })
    }
}

impl FromStr for ClawSolver {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = Regex::new(
            r"\s*Button A: X\+(?<ax>[0-9]+),\s*Y\+(?<ay>[0-9]+)\n\s*Button B: X\+(?<bx>[0-9]+), Y\+(?<by>[0-9]+)\n\s*Prize: X=(?<px>[0-9]+), Y=(?<py>[0-9]+)",
        )?.captures(s.trim()).ok_or("Invalid input")?;

        let ax = &caps["ax"].trim().parse()?;
        let ay = &caps["ay"].trim().parse()?;
        let bx = &caps["bx"].trim().parse()?;
        let by = &caps["by"].trim().parse()?;
        let px = &caps["px"].trim().parse()?;
        let py = &caps["py"].trim().parse()?;

        Ok(ClawSolver {
            ax: *ax,
            ay: *ay,
            bx: *bx,
            by: *by,
            px: *px,
            py: *py,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sm_solve() {
        let sm = "Button A: X+94, Y+34
                      Button B: X+22, Y+67
                      Prize: X=8400, Y=5400"
            .parse::<ClawSolver>()
            .unwrap();

        println!("{sm:?}");

        let x = sm.solve(0.0);

        assert_eq!(x, Some(vec![(80, 40)]));
    }

    #[test]
    fn test_sm_group_solve() {
        let total = "Button A: X+94, Y+34
                     Button B: X+22, Y+67
                     Prize: X=8400, Y=5400

                     Button A: X+26, Y+66
                     Button B: X+67, Y+21
                     Prize: X=12748, Y=12176

                     Button A: X+17, Y+86
                     Button B: X+84, Y+37
                     Prize: X=7870, Y=6450

                     Button A: X+69, Y+23
                     Button B: X+27, Y+71
                     Prize: X=18641, Y=10279"
            .split("\n\n")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|s| s.parse::<ClawSolver>().unwrap())
            .filter_map(|sm| {
                sm.solve(0.0)
                    .map(|x| x.into_iter().map(|(a, b)| a * 3 + b).min())?
            })
            .sum::<u64>();

        assert_eq!(total, 480);
    }

    #[test]
    fn test_sm_group_solve_offset() {
        let total = "Button A: X+94, Y+34
                     Button B: X+22, Y+67
                     Prize: X=8400, Y=5400

                     Button A: X+26, Y+66
                     Button B: X+67, Y+21
                     Prize: X=12748, Y=12176

                     Button A: X+17, Y+86
                     Button B: X+84, Y+37
                     Prize: X=7870, Y=6450

                     Button A: X+69, Y+23
                     Button B: X+27, Y+71
                     Prize: X=18641, Y=10279"
            .split("\n\n")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|s| s.parse::<ClawSolver>().unwrap())
            .filter_map(|sm| {
                sm.solve(10000000000000.0)
                    .map(|x| x.into_iter().map(|(a, b)| a * 3 + b).min())?
            })
            .sum::<u64>();

        assert_eq!(total, 875318608908);
    }
}
