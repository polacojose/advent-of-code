use std::str::FromStr;

use grid::{
    char::{TryFromChar, TryFromWhitespace},
    grid::Grid,
};

#[derive(Debug, Clone, Copy)]
pub enum OperationType {
    ADD,
    MUL,
}

#[derive(Debug, Clone, Copy)]
pub enum Node {
    Value(usize),
    Operation(OperationType),
}

#[derive(Debug, Clone, Copy)]
pub enum CharNode {
    Value(usize),
    Operation(OperationType),
    Blank,
}

impl FromStr for OperationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "+" => OperationType::ADD,
            "*" => OperationType::MUL,
            _ => return Err("Invalid operation".to_string()),
        })
    }
}

impl TryFromWhitespace for Node {
    fn from_str(s: &str, _row: usize, _col: usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(match s {
            "+" => Node::Operation(OperationType::ADD),
            "*" => Node::Operation(OperationType::MUL),
            v => Node::Value(v.to_string().parse()?),
        })
    }
}

impl TryFromChar for CharNode {
    fn from_char(c: char, _row: usize, _col: usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(match c {
            '+' => CharNode::Operation(OperationType::ADD),
            '*' => CharNode::Operation(OperationType::MUL),
            ' ' => CharNode::Blank,
            v => CharNode::Value(v.to_string().parse()?),
        })
    }
}

enum State {
    ReadOperands,
    ReadOperator,
    Done,
}

pub struct CethlapodMathReader {
    row: usize,
    col: usize,
    sum: usize,
    operands: Vec<usize>,
    grid: Grid<CharNode>,
}

impl CethlapodMathReader {
    pub fn new(grid: Grid<CharNode>) -> Self {
        Self {
            row: 0,
            col: grid.cols() - 1,
            sum: 0,
            operands: vec![],
            grid,
        }
    }

    pub fn sum(&mut self) -> usize {
        self.sum = 0;
        let mut state = State::ReadOperands;
        loop {
            state = match state {
                State::ReadOperands => self.read_operands(),
                State::ReadOperator => self.read_operator(),
                State::Done => break,
            }
        }
        self.sum
    }

    fn read_operands(&mut self) -> State {
        if self.row == 0 && self.col == 0 {
            return State::Done;
        }

        let mut operand = 0;

        loop {
            for i in 0..self.grid.rows() {
                self.row = i;
                let node = self.grid.get(self.row, self.col).unwrap();
                match node {
                    CharNode::Value(v) => {
                        operand = operand * 10 + v;
                    }
                    CharNode::Operation(_) => {
                        self.operands.push(operand);
                        return State::ReadOperator;
                    }
                    _ => {}
                }
            }

            self.operands.push(operand);
            self.col = self.col.saturating_sub(1);
            operand = 0;
        }
    }

    fn read_operator(&mut self) -> State {
        let operator = self.grid.get(self.row, self.col).unwrap();

        self.sum += self
            .operands
            .drain(..)
            .reduce(|acc, a| match operator {
                CharNode::Operation(op_type) => match op_type {
                    OperationType::ADD => acc + a,
                    OperationType::MUL => acc * a,
                },
                _ => acc,
            })
            .unwrap();

        self.row = 0;
        self.col = self.col.saturating_sub(2);

        return State::ReadOperands;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_ceph_sum() {
        let grid = TEST_DATA
            .parse::<Grid<CharNode>>()
            .expect("Unable to parse grid");

        let mut ceph = CethlapodMathReader::new(grid);
        assert_eq!(ceph.sum(), 3263827);
    }
}
