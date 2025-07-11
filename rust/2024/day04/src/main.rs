use std::fs;

use day04::{
    lettergrid::LetterGrid, lettergridsolver::solve_xmas_grid, letterkernel::LetterKernel,
};

const KERNEL_STRS: [&str; 4] = [
    r"M.S
      .A.
      M.S",
    r"S.S
      .A.
      M.M",
    r"S.M
      .A.
      S.M",
    r"M.M
      .A.
      S.S",
];
fn main() {
    let lg = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .parse::<LetterGrid>()
        .unwrap();
    part1(&lg);
    part2(&lg);
}

fn part1(lg: &LetterGrid) {
    let sum = solve_xmas_grid(&lg);
    println!("Part1: {sum}");
}

fn part2(lg: &LetterGrid) {
    let m = KERNEL_STRS
        .iter()
        .map(|tks| {
            let kernel = tks.parse::<LetterKernel>().unwrap();
            kernel.matches(&lg)
        })
        .sum::<u32>();
    println!("Part2: {m}");
}
