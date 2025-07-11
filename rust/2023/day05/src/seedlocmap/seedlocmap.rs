use std::io::{self, BufRead, BufReader, Lines, Read};

use super::{
    seedrange::SeedRange,
    sourcedestrangemap::{SourceDestRange, SourceDestRangeMap},
};

#[derive(Debug)]
pub struct SeedLocMap {
    seed_ranges: Vec<SeedRange>,
    seed_loc_map: Vec<SourceDestRange>,
}

impl SeedLocMap {
    pub fn new(readable: impl Read, seeds_ranged: bool) -> Self {
        let mut buffer = io::BufReader::new(readable).lines();

        let seed_line = buffer.next().unwrap().unwrap();
        let seed_ranges = SeedRange::parse_seed_ranges(seed_line, seeds_ranged);
        buffer.next();

        let source_dest_range_maps = Self::parse_source_dest_ranges(&mut buffer);
        let seed_loc_map = Self::compress_maps(source_dest_range_maps);

        Self {
            seed_ranges,
            seed_loc_map,
        }
    }
    pub fn lowest_seed_loc(&self) -> u32 {
        0
    }
}
impl SeedLocMap {
    pub(super) fn parse_source_dest_ranges(
        lines: &mut Lines<BufReader<impl Read>>,
    ) -> Vec<SourceDestRangeMap> {
        let mut source_dest_range_maps = Vec::new();
        while let Some(Ok(_)) = lines.next() {
            let mut source_dest_ranges = Vec::new();
            while let Some(Ok(line)) = lines.next() {
                if line.trim().is_empty() {
                    break;
                }
                source_dest_ranges.push(line.parse::<SourceDestRange>().unwrap());
            }

            source_dest_ranges.sort_unstable_by_key(|sdr| sdr.dest);

            if let Some(first) = source_dest_ranges.first() {
                if first.dest != 0 {
                    source_dest_ranges.insert(
                        0,
                        SourceDestRange {
                            source: 0,
                            dest: 0,
                            length: first.dest,
                        },
                    )
                }
            }

            source_dest_range_maps.push(SourceDestRangeMap {
                source_dest_ranges,
                seed: false,
            });
        }
        source_dest_range_maps
    }

    fn compress_maps(mut sourcedest_range_maps: Vec<SourceDestRangeMap>) -> Vec<SourceDestRange> {
        sourcedest_range_maps.reverse();
        sourcedest_range_maps
            .into_iter()
            .reduce(|child, parent| {
                let mut source_dest_ranges = Vec::new();

                child.source_dest_ranges.iter().for_each(|sdr| {
                    let mut working_sdr = sdr.clone();
                    loop {
                        if working_sdr.length <= 0 {
                            break;
                        }

                        let parent_range = parent.range_from_dest(working_sdr.source);
                        if parent_range.is_none() {
                            break;
                        }
                        let parent_range = parent_range.unwrap();

                        if parent_range.length < working_sdr.length {
                            source_dest_ranges.push(SourceDestRange {
                                dest: working_sdr.dest,
                                source: parent_range.source,
                                length: parent_range.length,
                            });
                            working_sdr = SourceDestRange {
                                dest: working_sdr.dest + parent_range.length,
                                source: working_sdr.source + parent_range.length,
                                length: working_sdr.length - parent_range.length,
                            }
                        } else {
                            source_dest_ranges.push(SourceDestRange {
                                dest: working_sdr.dest,
                                source: parent_range.source,
                                length: working_sdr.length,
                            });
                            working_sdr = SourceDestRange {
                                dest: 0,
                                source: 0,
                                length: 0,
                            }
                        }
                    }
                });

                return SourceDestRangeMap {
                    source_dest_ranges,
                    seed: false,
                };
            })
            .unwrap()
            .source_dest_ranges
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use stringreader::StringReader;

    use super::*;

    #[test]
    fn can_parse_source_dest_range() {
        let source_dest_range_maps = SeedLocMap::parse_source_dest_ranges(
            &mut BufReader::new(StringReader::new(
                r#"temperature-to-humidity map:
                0 69 1
                1 0 69

                humidity-to-location map:
                56 93 4"#,
            ))
            .lines(),
        );

        let compressed = SeedLocMap::compress_maps(source_dest_range_maps);
        assert_eq!(
            compressed,
            vec![
                SourceDestRange {
                    source: 69,
                    dest: 0,
                    length: 1
                },
                SourceDestRange {
                    source: 0,
                    dest: 1,
                    length: 55
                },
                SourceDestRange {
                    source: 93,
                    dest: 56,
                    length: 4
                }
            ]
        );
        //println!("Compressed\n{:?}", compressed);
    }

    #[test]
    fn can_find_lowest_location() {
        let seed_loc_map = SeedLocMap::new(fs::File::open("test-input.txt").unwrap(), false);
        assert_eq!(seed_loc_map.lowest_seed_loc(), 35);

        let seed_loc_map = SeedLocMap::new(fs::File::open("test-input.txt").unwrap(), true);
        assert_eq!(seed_loc_map.lowest_seed_loc(), 46);
    }
}
