mod nav_reader;
mod nav_tree;
mod string_reader;

use std::fs::File;

use nav_reader::NavReader;
use nav_tree::NavTree;

fn main() {
    let mut nav_reader = NavReader::new(File::open("input.txt").unwrap()).unwrap();
    let mut nav_tree = NavTree::new(&mut nav_reader);
    let tree = nav_tree.parse_tree().unwrap();
    println!("Metadata Sum: {}", tree.sum_metadata());
    println!("Metadata Sum By Index: {}", tree.sum_metadata_by_index());
}
