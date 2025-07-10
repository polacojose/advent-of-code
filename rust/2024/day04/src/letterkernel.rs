use std::{io, str::FromStr};

use crate::{letter::Letter, lettergrid::LetterGrid, vector::IVec2};

struct LetterKernelPiece {
    delta: IVec2,
    letter: Letter,
}

pub struct LetterKernel {
    kernels: Vec<LetterKernelPiece>,
}

impl FromStr for LetterKernel {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kernels = s.trim().lines().enumerate().fold(
            Vec::<LetterKernelPiece>::new(),
            |mut acc: Vec<LetterKernelPiece>, (y, a): (usize, &str)| {
                let y = y as isize;
                let mut letters: Vec<LetterKernelPiece> = a
                    .trim()
                    .chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        let x = x as isize;
                        match c {
                            'X' => Some(LetterKernelPiece {
                                delta: IVec2 { x, y },
                                letter: Letter::X,
                            }),
                            'M' => Some(LetterKernelPiece {
                                delta: IVec2 { x, y },
                                letter: Letter::M,
                            }),
                            'A' => Some(LetterKernelPiece {
                                delta: IVec2 { x, y },
                                letter: Letter::A,
                            }),
                            'S' => Some(LetterKernelPiece {
                                delta: IVec2 { x, y },
                                letter: Letter::S,
                            }),
                            _ => None,
                        }
                    })
                    .collect();
                acc.append(&mut letters);
                acc
            },
        );

        Ok(LetterKernel { kernels })
    }
}

impl LetterKernel {
    pub fn matches(&self, lg: &LetterGrid) -> u32 {
        let mut matches = 0;

        for y in 0..lg.height as isize {
            for x in 0..lg.width as isize {
                if self.kernels.iter().all(|k| {
                    let a = IVec2 { x, y } + k.delta;

                    if let Some(lg_letter) = lg.get_letter(a) {
                        &k.letter == lg_letter
                    } else {
                        false
                    }
                }) {
                    matches += 1;
                }
            }
        }

        matches
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_KERNEL_STRS: [&str; 4] = [
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

    const TEST_STR: &str = r"MMMSXXMASM
                             MSAMXMSMSA
                             AMXSXMAAMM
                             MSAMASMSMX
                             XMASAMXAMM
                             XXAMMXXAMA
                             SMSMSASXSS
                             SAXAMASAAA
                             MAMMMXMMMM
                             MXMXAXMASX";

    #[test]
    fn test_kernel_matches() {
        let m = TEST_KERNEL_STRS
            .iter()
            .map(|tks| {
                let kernel = tks.parse::<LetterKernel>().unwrap();
                let lg = TEST_STR.parse::<LetterGrid>().unwrap();
                kernel.matches(&lg)
            })
            .sum::<u32>();
        assert_eq!(m, 9);
    }
}
