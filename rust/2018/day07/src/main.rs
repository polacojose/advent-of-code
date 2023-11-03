use std::fs;

use deptree::DepTree;

use crate::job::{Job, JobConfig};

mod deptree;
mod job;

fn main() {
    part1();
    part2();
}

fn part1() {
    let deptree: DepTree = fs::read_to_string("input.txt").unwrap().parse().unwrap();
    println!("StepOrder: {}", deptree.dependency_order());
}

fn part2() {
    let deptree: DepTree = fs::read_to_string("input.txt").unwrap().parse().unwrap();
    let mut job = Job::new(JobConfig {
        dep_tree: deptree,
        workers: 5,
        base_seconds: 60,
    });
    println!("Seconds to complete: {}", job.complete_job());
}
