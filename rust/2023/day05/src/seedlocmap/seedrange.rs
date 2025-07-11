#[derive(Debug, PartialEq, Eq)]
pub(super) struct SeedRange {
    start: u32,
    length: u32,
}

impl SeedRange {
    pub(super) fn parse_seed_ranges(
        mut line: impl AsRef<str>,
        seeds_ranged: bool,
    ) -> Vec<SeedRange> {
        let seed_nums = line
            .as_ref()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace();

        let seed_ranges = seed_nums
            .map(|seed_num| seed_num.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if !seeds_ranged {
            return seed_ranges
                .into_iter()
                .map(|seed_num| SeedRange {
                    start: seed_num,
                    length: 1,
                })
                .collect();
        }

        return seed_ranges
            .chunks(2)
            .map(|num_pair| SeedRange {
                start: num_pair[0],
                length: num_pair[1],
            })
            .collect();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_SEEDS: &str = "seeds: 79 14 55 13";

    #[test]
    fn can_parse_seed_ranges() {
        let seed_ranges = SeedRange::parse_seed_ranges(TEST_SEEDS, false);

        assert_eq!(
            seed_ranges,
            vec![
                SeedRange {
                    start: 79,
                    length: 1
                },
                SeedRange {
                    start: 14,
                    length: 1
                },
                SeedRange {
                    start: 55,
                    length: 1
                },
                SeedRange {
                    start: 13,
                    length: 1
                },
            ],
        );

        let seed_ranges = SeedRange::parse_seed_ranges(TEST_SEEDS, true);

        assert_eq!(
            seed_ranges,
            vec![
                SeedRange {
                    start: 79,
                    length: 14
                },
                SeedRange {
                    start: 55,
                    length: 13
                },
            ],
        );
    }
}
