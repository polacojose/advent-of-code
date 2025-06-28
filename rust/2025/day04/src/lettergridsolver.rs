use crate::{letter::Letter, lettergrid::LetterGrid, vector::IVec2};

pub fn solve_xmas_grid(lg: &LetterGrid) -> u32 {
    let mut sums = 0;

    for y in 0..lg.height as isize {
        for x in 0..lg.width as isize {
            let p = IVec2 { x, y };
            match lg.get_letter(p) {
                Some(l) => match l {
                    Letter::X => {
                        let mut s = 0;

                        //Deltas
                        for dx in -1_isize..=1 {
                            for dy in -1_isize..=1 {
                                if dx == 0 && dy == 0 {
                                    continue;
                                }

                                let d = IVec2 { x: dx, y: dy };
                                s += complete_tails(&lg, l, p, d);
                            }
                        }
                        sums += s;
                    }
                    _ => (),
                },
                None => (),
            };
        }
    }

    sums
}

fn complete_tails(lg: &LetterGrid, letter: &Letter, p: IVec2, d: IVec2) -> u32 {
    if d.x == 0 && d.y == 0 {
        return 0;
    }

    if let Some(next_letter) = letter.next() {
        let mut sums = 0;

        let a = p + d;
        if let Some(adj_letter) = lg.get_letter(a) {
            if *adj_letter == next_letter {
                sums += complete_tails(lg, adj_letter, a, d)
            }
        }

        sums
    } else {
        return 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const MINI_TEST_STR: &str = r"MMMSX
                                  MSAMX";

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
    fn test_solve_grid() {
        let lg = MINI_TEST_STR.parse::<LetterGrid>().unwrap();
        let sum = solve_xmas_grid(&lg);
        assert_eq!(sum, 1);

        let lg = TEST_STR.parse::<LetterGrid>().unwrap();
        let sum = solve_xmas_grid(&lg);
        assert_eq!(sum, 18);
    }
}
