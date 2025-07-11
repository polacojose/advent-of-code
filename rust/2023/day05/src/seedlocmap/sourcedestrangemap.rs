use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) struct SourceDestRange {
    pub(super) source: u32,
    pub(super) dest: u32,
    pub(super) length: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct SourceDestRangeMap {
    pub(super) seed: bool,
    pub(super) source_dest_ranges: Vec<SourceDestRange>,
}

#[derive(Debug)]
pub(super) struct UnableToParse;

impl FromStr for SourceDestRange {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split_whitespace().collect::<Vec<&str>>();
        Ok(SourceDestRange {
            source: parts[1].parse::<u32>().unwrap(),
            dest: parts[0].parse::<u32>().unwrap(),
            length: parts[2].parse::<u32>().unwrap(),
        })
    }
}

impl SourceDestRangeMap {
    pub(super) fn range_from_dest(&self, dest: u32) -> Option<SourceDestRange> {
        for sdr in self.source_dest_ranges.iter() {
            if sdr.dest <= dest && sdr.dest.saturating_add(sdr.length) > dest {
                let diff = dest - sdr.dest;
                return Some(SourceDestRange {
                    source: sdr.source + diff,
                    dest: sdr.dest + diff,
                    length: sdr.length - diff,
                });
            }
        }

        if !self.seed {
            return Some(SourceDestRange {
                source: dest,
                dest,
                length: u32::MAX,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seedlocmap::seedlocmap::SeedLocMap;
    use std::io::{BufRead, BufReader};
    use stringreader::StringReader;

    #[test]
    fn can_parse_source_dest_range() {
        let source_dest_range = SourceDestRange::from_str("79 14 55").unwrap();
        assert_eq!(
            source_dest_range,
            SourceDestRange {
                source: 14,
                dest: 79,
                length: 55
            }
        );
    }

    #[test]
    fn can_parse_range_map() {
        let source_dest_range_maps = SeedLocMap::parse_source_dest_ranges(
            &mut BufReader::new(StringReader::new(
                r#"temperature-to-humidity map:
                0 69 1
                1 0 69

                humidity-to-location map:
                60 56 37
                56 93 4"#,
            ))
            .lines(),
        );
        assert_eq!(
            source_dest_range_maps,
            vec![
                SourceDestRangeMap {
                    seed: false,
                    source_dest_ranges: vec![
                        SourceDestRange {
                            source: 69,
                            dest: 0,
                            length: 1
                        },
                        SourceDestRange {
                            source: 0,
                            dest: 1,
                            length: 69
                        },
                    ]
                },
                SourceDestRangeMap {
                    seed: false,
                    source_dest_ranges: vec![
                        SourceDestRange {
                            source: 0,
                            dest: 0,
                            length: 56
                        },
                        SourceDestRange {
                            source: 93,
                            dest: 56,
                            length: 4
                        },
                        SourceDestRange {
                            source: 56,
                            dest: 60,
                            length: 37
                        },
                    ]
                }
            ]
        );
    }

    macro_rules! assert_dest_range {
        ($source_dest_range_map:expr, $dest:expr, $s:expr, $d:expr, $l:expr) => {
            assert_eq!(
                $source_dest_range_map.range_from_dest($dest),
                Some(SourceDestRange {
                    source: $s,
                    dest: $d,
                    length: $l
                })
            );
        };
    }

    #[test]
    fn can_find_range() {
        let source_dest_range_map = SeedLocMap::parse_source_dest_ranges(
            &mut BufReader::new(StringReader::new(
                r#"temperature-to-humidity map:
                0 69 1
                1 0 69"#,
            ))
            .lines(),
        )
        .into_iter()
        .next()
        .unwrap();

        assert_dest_range!(source_dest_range_map, 0, 69, 0, 1);
        assert_dest_range!(source_dest_range_map, 50, 49, 50, 20);
        assert_dest_range!(source_dest_range_map, 70, 70, 70, u32::MAX);
        assert_dest_range!(source_dest_range_map, 100, 100, 100, u32::MAX);
    }
}
