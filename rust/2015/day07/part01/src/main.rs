use std::fs;

use part01::{OperationClass, WIRE_VALUES};

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut operations: Vec<OperationClass> = Vec::new();
    for line in file_string.lines() {
        operations.push(line.parse().unwrap());
    }

    OperationClass::store_operations(operations);
    OperationClass::resolve_operations();

    println!("{:#?}", WIRE_VALUES.read().unwrap());
}
