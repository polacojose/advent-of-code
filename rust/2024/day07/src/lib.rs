use std::{io, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Mul,
    Comp,
}

fn eval(operands: &[u64], operators: &[Operator]) -> u64 {
    let acc = *operands.first().unwrap_or(&0);
    operands
        .iter()
        .skip(1)
        .zip(operators)
        .fold(acc, |acc, (a, b)| match b {
            Operator::Add => acc + a,
            Operator::Mul => acc * a,
            Operator::Comp => concat(acc, *a),
        })
}

#[inline]
fn concat(acc: u64, a: u64) -> u64 {
    let mut multiplier = 1;
    let mut temp = a;
    while temp > 0 {
        multiplier *= 10;
        temp /= 10;
    }
    acc * multiplier + a
}

pub fn find_operators(
    solution: u64,
    operands: &[u64],
    available_operators: &[Operator],
) -> Result<Vec<Vec<Operator>>, ()> {
    f_o(solution, operands, &mut vec![], available_operators)
}

fn f_o(
    solution: u64,
    operands: &[u64],
    operators: &mut Vec<Operator>,
    available_operators: &[Operator],
) -> Result<Vec<Vec<Operator>>, ()> {
    if operators.len() >= 1 {
        let s = eval(&operands, &operators);
        if s > solution {
            return Err(());
        }
        if operators.len() == operands.len() - 1 {
            if s == solution {
                return Ok(vec![operators.clone()]);
            } else {
                return Err(());
            }
        }
    }

    Ok(available_operators
        .iter()
        .filter_map(|o| {
            //let mut ops = operators.clone();
            //ops.push(*o);
            operators.push(*o);
            let a = f_o(solution, operands, operators, available_operators).ok();
            operators.pop();
            a
        })
        .flat_map(|v| v)
        .collect())
}

pub struct Equation {
    pub solution: u64,
    pub operands: Vec<u64>,
}

impl FromStr for Equation {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (solution_str, operands_str) = s.trim().split_once(":").ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "Incorrect Equation Format",
        ))?;

        let solution = solution_str
            .trim()
            .parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let operands = operands_str
            .trim()
            .split_whitespace()
            .map(|s| {
                s.parse::<u64>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .collect::<Result<Vec<u64>, Self::Err>>()?;

        Ok(Self { solution, operands })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_STR: &str = r"190: 10 19
                             3267: 81 40 27
                             83: 17 5
                             156: 15 6
                             7290: 6 8 6 15
                             161011: 16 10 13
                             192: 17 8 14
                             21037: 9 7 18 13
                             292: 11 6 16 20";

    macro_rules! test_operator_search {
        ($sol:expr, $operands:expr, $operators:expr, $available_operators:expr) => {
            let split_opts: Vec<Vec<Operator>> = $operators
                .iter()
                .filter_map(|s| {
                    let s = s.trim();
                    if s.is_empty() { None } else { Some(s) }
                })
                .map(|s| s.trim().split_whitespace().collect::<Vec<_>>())
                .filter_map(|s| {
                    s.iter()
                        .map(|t| match t.trim() {
                            "Add" => Some(Operator::Add),
                            "Mul" => Some(Operator::Mul),
                            _ => None,
                        })
                        .collect()
                })
                .collect();

            let result = find_operators($sol, $operands, $available_operators);

            if $operators.len() > 0 {
                assert_eq!(result, Ok(split_opts));
            } else {
                assert_eq!(result, Err(()));
            }
        };
    }

    #[test]
    fn test_eval() {
        let solution = eval(&mut vec![10, 19], &vec![Operator::Mul]);
        println!("{solution}");
        assert_eq!(solution, 190);

        let solution = eval(&mut vec![10, 19], &vec![Operator::Comp]);
        println!("{solution}");
        assert_eq!(solution, 1019);
    }

    #[test]
    fn test_operator_search() {
        test_operator_search!(190, &[10, 19], ["Mul"], &[Operator::Add, Operator::Mul]);
        test_operator_search!(
            3267,
            &[81, 40, 27],
            ["Add Mul", "Mul Add"],
            &[Operator::Add, Operator::Mul]
        );
        test_operator_search!(83, &[17, 5], [""], &[Operator::Add, Operator::Mul]);
        test_operator_search!(156, &[15, 6], [""], &[Operator::Add, Operator::Mul]);
        test_operator_search!(7290, &[6, 8, 6, 15], [""], &[Operator::Add, Operator::Mul]);
        test_operator_search!(
            1611011,
            &[16, 10, 13],
            [""],
            &[Operator::Add, Operator::Mul]
        );
        test_operator_search!(192, &[17, 8, 14], [""], &[Operator::Add, Operator::Mul]);
        test_operator_search!(
            21037,
            &[9, 7, 18, 13],
            [""],
            &[Operator::Add, Operator::Mul]
        );
        test_operator_search!(
            292,
            &[11, 6, 16, 20],
            ["Add Mul Add"],
            &[Operator::Add, Operator::Mul]
        );
    }

    #[test]
    fn test_correct_total() {
        let p1_operators = [Operator::Add, Operator::Mul];
        let s: u64 = TEST_STR
            .lines()
            .filter_map(|l| {
                let e = l.parse::<Equation>().unwrap();
                if let Ok(operands) = find_operators(e.solution, &e.operands, &p1_operators) {
                    if !operands.is_empty() {
                        Some(e.solution)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum();

        assert_eq!(s, 3749);

        let p2_operators = [Operator::Add, Operator::Mul, Operator::Comp];
        let s: u64 = TEST_STR
            .lines()
            .filter_map(|l| {
                let e = l.parse::<Equation>().unwrap();
                if let Ok(operands) = find_operators(e.solution, &e.operands, &p2_operators) {
                    if !operands.is_empty() {
                        Some(e.solution)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum();

        assert_eq!(s, 11387);
    }
}
