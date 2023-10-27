use std::{fmt::Display, str::FromStr};

use crate::rule::UnableToParse;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pattern(pub Vec<Vec<char>>);

#[derive(Debug)]
pub struct UnableToSubdivide;

impl Pattern {
    pub(crate) fn match_other(&self, other: &Pattern) -> bool {
        self.rotations().into_iter().any(|x| x == *other)
    }

    fn rotations(&self) -> Vec<Pattern> {
        let up = self.clone();
        let up_flipped = Pattern::flip_x(&up);
        let down = Pattern::flip_x(&Pattern::flip_y(self));
        let down_flipped = Pattern::flip_y(self);

        let left = Pattern::flip_y(&Pattern::transpose(self));
        let left_flipped = Pattern::transpose(self);
        let right = Pattern::flip_x(&Pattern::transpose(self));
        let right_flipped = Pattern::flip_y(&right);

        vec![
            up,
            up_flipped,
            down,
            down_flipped,
            left,
            left_flipped,
            right,
            right_flipped,
        ]
    }

    pub fn root_patterns(&self) -> Result<Vec<Pattern>, UnableToSubdivide> {
        if self.0.len() <= 3 {
            return Err(UnableToSubdivide);
        }

        for n in (2..=3).rev() {
            if self.0.len() % n != 0 {
                continue;
            }

            let sub_size = self.0.len() / n;
            let mut sub_patterns = vec![];
            for i in 0..self.0.len() / sub_size {
                for j in 0..self.0.len() / sub_size {
                    sub_patterns.push(self.extract_sub_pattern(
                        sub_size,
                        j * sub_size,
                        i * sub_size,
                    ));
                }
            }

            return Ok(sub_patterns);
        }

        return Err(UnableToSubdivide);
    }

    fn extract_sub_pattern(&self, sub_size: usize, x: usize, y: usize) -> Pattern {
        let mut contents = vec![vec!['.'; sub_size]; sub_size];
        for i in 0..sub_size {
            for j in 0..sub_size {
                contents[i][j] = self.0[i + y][j + x];
            }
        }
        Pattern(contents)
    }

    pub fn matches(&self, c: char) -> usize {
        self.0.iter().flatten().filter(|x| **x == c).count()
    }
}

impl Pattern {
    pub fn quad(sub_patterns: &[Pattern]) -> Pattern {
        let mut sub_patterns = sub_patterns.to_owned();
        let row_size = (sub_patterns.len() as f64).sqrt() as usize;

        let mut pattern_contents = Vec::new();
        for y in 0..row_size {
            let mut row = vec![vec![]; sub_patterns[0].0.len()];
            for x in 0..row_size {
                for yy in 0..sub_patterns[0].0.len() {
                    row[yy].append(&mut sub_patterns[x + y * row_size].0[yy])
                }
            }
            pattern_contents.append(&mut row);
        }

        Pattern(pattern_contents)
    }

    fn transpose(pattern: &Pattern) -> Pattern {
        let (width, height) = (pattern.0.len(), pattern.0[0].len());
        let mut transposed = vec![vec!['.'; width]; height];
        for i in 0..height {
            for j in 0..width {
                transposed[j][i] = pattern.0[i][j];
            }
        }
        Pattern(transposed)
    }

    fn flip_x(pattern: &Pattern) -> Pattern {
        Pattern(
            pattern
                .0
                .iter()
                .map(|x| x.iter().rev().copied().collect())
                .collect(),
        )
    }

    fn flip_y(pattern: &Pattern) -> Pattern {
        Pattern(pattern.0.clone().into_iter().rev().collect())
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for c in row {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl FromStr for Pattern {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let content = s
            .trim()
            .split("/")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| x.chars().collect())
            .collect();
        Ok(Pattern(content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose() {
        let pattern_a = Pattern(vec![
            vec!['.', '#', '#'],
            vec!['.', '#', '.'],
            vec!['.', '#', '.'],
        ]);
        let pattern_b = Pattern(vec![
            vec!['.', '.', '.'],
            vec!['#', '#', '#'],
            vec!['#', '.', '.'],
        ]);
        assert_eq!(Pattern::transpose(&pattern_a), pattern_b);
    }

    #[test]
    fn flip_x() {
        let pattern_a = Pattern(vec![
            vec!['.', '#', '#'],
            vec!['.', '#', '.'],
            vec!['.', '#', '.'],
        ]);
        let pattern_b = Pattern(vec![
            vec!['#', '#', '.'],
            vec!['.', '#', '.'],
            vec!['.', '#', '.'],
        ]);
        assert_eq!(Pattern::flip_x(&pattern_a), pattern_b);
    }

    #[test]
    fn flip_y() {
        let pattern_a = Pattern(vec![
            vec!['.', '#', '#'],
            vec!['.', '#', '.'],
            vec!['.', '#', '.'],
        ]);
        let pattern_b = Pattern(vec![
            vec!['.', '#', '.'],
            vec!['.', '#', '.'],
            vec!['.', '#', '#'],
        ]);
        assert_eq!(Pattern::flip_y(&pattern_a), pattern_b);
    }
}
