pub mod vert;

use std::fs;

use grid::grid::Grid;

use crate::vert::{CethlapodMathReader, CharNode, Node, OperationType};

fn main() {
    let grid = Grid::<Node>::from_str_whitespace(fs::read_to_string("input").unwrap().trim())
        .expect("Unable to parse grid");

    let total = (0..grid.cols()).into_iter().fold(0, |acc, x| {
        let (operands, operation_type) =
            (0..grid.rows())
                .into_iter()
                .fold((vec![], None), |(mut operands, op_type), y| {
                    match grid[(y, x)] {
                        Node::Value(v) => {
                            operands.push(v);
                            (operands, op_type)
                        }
                        Node::Operation(ot) => (operands, Some(ot)),
                    }
                });

        let op_type = operation_type.unwrap();

        acc + operands
            .into_iter()
            .reduce(|a, n| match op_type {
                OperationType::ADD => a + n,
                OperationType::MUL => a * n,
            })
            .unwrap()
    });

    println!("Part 1: {total}");

    let grid = fs::read_to_string("input")
        .unwrap()
        .parse::<Grid<CharNode>>()
        .expect("Unable to parse grid");

    let mut ceph = CethlapodMathReader::new(grid);
    println!("Print 2: {}", ceph.sum());
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = r#"123 328  51 64 
45 64  387 23 
6 98  215 314
*   +   *   +"#;

    #[test]
    fn test_operations() {
        let grid =
            Grid::<Node>::from_str_whitespace(TEST_DATA.trim()).expect("Unable to parse grid");

        let total =
            (0..grid.cols()).into_iter().fold(0, |acc, x| {
                let (operands, operation_type) = (0..grid.rows()).into_iter().fold(
                    (vec![], None),
                    |(mut operands, op_type), y| match grid[(y, x)] {
                        Node::Value(v) => {
                            operands.push(v);
                            (operands, op_type)
                        }
                        Node::Operation(ot) => (operands, Some(ot)),
                    },
                );

                let op_type = operation_type.unwrap();

                acc + operands
                    .into_iter()
                    .reduce(|a, n| match op_type {
                        OperationType::ADD => a + n,
                        OperationType::MUL => a * n,
                    })
                    .unwrap()
            });

        assert_eq!(total, 4277556);
    }
}
