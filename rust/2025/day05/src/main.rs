use std::{error::Error, fs, str::FromStr};

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl FromStr for Range {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").unwrap();
        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

impl Range {
    pub fn contains(&self, n: impl Into<usize>) -> bool {
        let n = n.into();
        self.start <= n && n <= self.end
    }

    pub fn length(&self) -> usize {
        if self.start > self.end {
            return 0;
        }

        self.end - self.start + 1
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let (fresh_ranges, ids) = input.split_once("\n\n").expect("Unable to read data");

    let fresh_ranges: Vec<Range> = fresh_ranges
        .trim()
        .lines()
        .map(|l| l.trim().parse())
        .collect::<Result<Vec<Range>, _>>()
        .expect("Unable to parse ranges");

    let fresh = part1(ids, &fresh_ranges);

    println!("Part 1: {fresh}");

    let fresh = part2(fresh_ranges);

    println!("Part 2: {fresh}");
}

fn part2(mut fresh_ranges: Vec<Range>) -> usize {
    fresh_ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let (_, fresh) = fresh_ranges.into_iter().fold(
        (None, 0),
        |(last_range, acc): (Option<Range>, usize), mut a| {
            if let Some(last) = &last_range {
                a.start = a.start.max(last.end + 1);
            }

            if a.length() == 0 {
                (last_range, acc)
            } else {
                (Some(a.clone()), acc + a.length())
            }
        },
    );
    fresh
}

fn part1(ids: &str, fresh_ranges: &Vec<Range>) -> usize {
    let ids: Vec<usize> = ids
        .trim()
        .lines()
        .map(|l| l.trim().parse())
        .collect::<Result<Vec<usize>, _>>()
        .expect("Unable to parse ranges");

    let fresh = ids
        .into_iter()
        .filter(|id| fresh_ranges.iter().any(|range| range.contains(*id)))
        .count();
    fresh
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "3-5
                            10-14
                            16-20
                            12-18

                            1
                            5
                            8
                            11
                            17
                            32";

    #[test]
    fn test_range_parse() {
        let (fresh_ranges, ids) = TEST_DATA.split_once("\n\n").expect("Unable to read data");

        let fresh_ranges: Vec<Range> = fresh_ranges
            .trim()
            .lines()
            .map(|l| l.trim().parse())
            .collect::<Result<Vec<Range>, _>>()
            .expect("Unable to parse ranges");

        let ids: Vec<usize> = ids
            .trim()
            .lines()
            .map(|l| l.trim().parse())
            .collect::<Result<Vec<usize>, _>>()
            .expect("Unable to parse ranges");

        let fresh = ids
            .into_iter()
            .filter(|id| fresh_ranges.iter().any(|range| range.contains(*id)))
            .count();

        assert_eq!(fresh, 3);
    }

    #[test]
    fn test_range_length() {
        let (fresh_ranges, _) = TEST_DATA.split_once("\n\n").expect("Unable to read data");

        let mut fresh_ranges: Vec<Range> = fresh_ranges
            .trim()
            .lines()
            .map(|l| l.trim().parse())
            .collect::<Result<Vec<Range>, _>>()
            .expect("Unable to parse ranges");

        fresh_ranges.sort_by(|a, b| a.start.cmp(&b.start));

        let (_, fresh) = fresh_ranges.into_iter().fold(
            (None, 0),
            |(last_range, acc): (Option<Range>, usize), mut a| {
                if let Some(last) = &last_range {
                    a.start = a.start.max(last.end + 1);
                }

                println!("a: {a:#?}");

                if a.length() == 0 {
                    (last_range, acc)
                } else {
                    (Some(a.clone()), acc + a.length())
                }
            },
        );

        assert_eq!(fresh, 14);
    }
}
