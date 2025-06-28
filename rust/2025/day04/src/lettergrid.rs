use std::{
    io::{self, Error},
    str::FromStr,
};

use crate::{letter::Letter, vector::IVec2};

pub struct LetterGrid {
    pub width: usize,
    pub height: usize,
    pub letters: Vec<Letter>,
}

impl FromStr for LetterGrid {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.trim().lines().count();

        let (width, letters) = s.trim().lines().fold(
            Ok((0, Vec::new())),
            |state, a: &str| -> Result<(usize, Vec<Letter>), io::Error> {
                if let Ok((_, mut l)) = state {
                    let mut letters = a
                        .trim()
                        .chars()
                        .map(|c| match c {
                            'X' => Ok(Letter::X),
                            'M' => Ok(Letter::M),
                            'A' => Ok(Letter::A),
                            'S' => Ok(Letter::S),
                            _ => {
                                return Err(Error::new(
                                    io::ErrorKind::Unsupported,
                                    "Invalid Characters {c}",
                                ));
                            }
                        })
                        .collect::<Result<Vec<Letter>, io::Error>>()?;
                    let width = letters.len();
                    l.append(&mut letters);
                    Ok((width, l))
                } else {
                    state
                }
            },
        )?;

        Ok(LetterGrid {
            width,
            height,
            letters,
        })
    }
}

impl LetterGrid {
    pub fn get_letter(&self, pos: IVec2) -> Option<&Letter> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.width as isize || pos.y >= self.height as isize {
            return None;
        }

        let x = pos.x as usize;
        let y = pos.y as usize;

        let idx = y * self.width + x;
        self.letters.get(idx)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const MINI_TEST_STR: &str = r"MMM
                                  MSA
                                  AMX";

    #[test]
    fn test_parse_grid() {
        let lg = MINI_TEST_STR.parse::<LetterGrid>().unwrap();
        assert_eq!(lg.width, 3);
        assert_eq!(lg.height, 3);

        #[rustfmt::skip]
        assert_eq!(
            lg.letters,
            [
                Letter::M, Letter::M, Letter::M,
                Letter::M, Letter::S, Letter::A,
                Letter::A, Letter::M, Letter::X
            ]
        );
    }

    #[test]
    fn test_get_letter() {
        let lg = MINI_TEST_STR.parse::<LetterGrid>().unwrap();
        assert!(matches!(
            lg.get_letter(IVec2 { x: 0, y: 0 }),
            Some(&Letter::M)
        ));
        assert!(matches!(
            lg.get_letter(IVec2 { x: 1, y: 1 }),
            Some(&Letter::S)
        ));
        assert!(matches!(
            lg.get_letter(IVec2 { x: 2, y: 2 }),
            Some(&Letter::X)
        ));
    }
}
