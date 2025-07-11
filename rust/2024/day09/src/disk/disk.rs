#[derive(Debug, Clone, Copy)]
pub(super) enum DiskEntryKind {
    Data(usize),
    Empty,
}

#[derive(Debug)]
pub(super) struct DiskEntry {
    pub(super) length: usize,
    pub(super) kind: DiskEntryKind,
}

pub struct Disk {
    pub(super) disk_entries: Vec<DiskEntry>,
}

impl Disk {
    pub fn checksum(&mut self) -> usize {
        let (_, sum) = self
            .disk_entries
            .iter()
            .fold((0, 0), |(mut idx, mut sum), e| {
                match e.kind {
                    DiskEntryKind::Data(id) => {
                        for idx in idx..idx + e.length {
                            sum += idx * id;
                        }
                        idx += e.length;
                    }
                    DiskEntryKind::Empty => {
                        idx += e.length;
                    }
                }

                (idx, sum)
            });
        sum
    }

    pub fn quick_defrag(&mut self) -> Option<()> {
        let mut last_data_id = self.disk_entries.len();
        loop {
            let data_entry_idx = self.last_data_entry_by_id_id(last_data_id)?;
            let length = self.disk_entries[data_entry_idx].length;

            let space_idx = self.next_free_space_of_length_id(length);

            if space_idx.is_none() {
                last_data_id = last_data_id - 1;
                continue;
            }
            let space_idx = space_idx?;

            if space_idx >= data_entry_idx {
                last_data_id = last_data_id.saturating_sub(1);
                if last_data_id == 0 {
                    return None;
                }
                continue;
            }

            let data_length = self.disk_entries[data_entry_idx].length;
            self.disk_entries[space_idx].length -= data_length;

            match self.disk_entries[data_entry_idx].kind {
                DiskEntryKind::Data(x) => last_data_id = x - 1,
                _ => (),
            }

            //Insert data entry
            self.disk_entries.insert(
                space_idx,
                DiskEntry {
                    length: data_length,
                    kind: self.disk_entries[data_entry_idx].kind,
                },
            );
            self.disk_entries[data_entry_idx + 1].kind = DiskEntryKind::Empty;
        }
    }

    pub fn frag_compact(&mut self) -> Option<()> {
        let mut space_offset = 0;
        let mut data_offset = self.disk_entries.len();

        loop {
            let space_idx = self.next_free_space_id(space_offset)?;
            let data_entry_idx = self.last_data_entry_id(data_offset)?;

            if space_idx >= data_entry_idx {
                //Empty last entries
                self.disk_entries
                    .iter_mut()
                    .filter(|e| matches!(e.kind, DiskEntryKind::Empty))
                    .for_each(|e| e.length = 0);
                return None;
            }

            //Shrink min from space and data
            let min = self.disk_entries[space_idx]
                .length
                .min(self.disk_entries[data_entry_idx].length);

            self.disk_entries[space_idx].length -= min;
            self.disk_entries[data_entry_idx].length -= min;
            if self.disk_entries[space_idx].length == 0 {
                space_offset = space_idx + 1;
            }
            if self.disk_entries[data_entry_idx].length == 0 {
                data_offset = data_entry_idx;
            }

            //Insert min data entry
            self.disk_entries.insert(
                space_idx,
                DiskEntry {
                    length: min,
                    kind: self.disk_entries[data_entry_idx].kind,
                },
            );
            data_offset += 1;
        }
    }

    fn next_free_space_id(&mut self, start: usize) -> Option<usize> {
        self.disk_entries[start..]
            .iter()
            .enumerate()
            .find_map(|(i, e)| {
                if matches!(e.kind, DiskEntryKind::Empty) && e.length > 0 {
                    Some(i + start)
                } else {
                    None
                }
            })
    }

    fn next_free_space_of_length_id(&mut self, length: usize) -> Option<usize> {
        self.disk_entries.iter().enumerate().find_map(|(i, e)| {
            if matches!(e.kind, DiskEntryKind::Empty) && e.length > 0 && e.length >= length {
                Some(i)
            } else {
                None
            }
        })
    }

    fn last_data_entry_id(&mut self, end: usize) -> Option<usize> {
        self.disk_entries[..end]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, e)| {
                if !matches!(e.kind, DiskEntryKind::Empty) && e.length > 0 {
                    Some(i)
                } else {
                    None
                }
            })
    }
    fn last_data_entry_by_id_id(&mut self, id: usize) -> Option<usize> {
        self.disk_entries
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, e)| {
                if e.length > 0 {
                    match e.kind {
                        DiskEntryKind::Data(x) => {
                            if x <= id {
                                Some(i)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const START_DISK: &str = "12345";
    const COMPACTED_DISK_LAYOUT: &str = "022111222";

    const START_DISK_2: &str = "2333133121414131402";
    const COMPACTED_DISK_LAYOUT_2: &str = "0099811188827773336446555566";

    const COMPACTED_DISK_LAYOUT_3: &str = "00992111777.44.333....5555.6666.....8888..";

    #[test]
    pub fn test_compact() {
        let mut disk = START_DISK.parse::<Disk>().unwrap();
        disk.frag_compact();
        assert_eq!(format!("{}", disk), COMPACTED_DISK_LAYOUT);

        let mut disk = START_DISK_2.parse::<Disk>().unwrap();
        disk.frag_compact();
        assert_eq!(format!("{}", disk), COMPACTED_DISK_LAYOUT_2);
    }

    #[test]
    pub fn test_checksum() {
        let mut disk = START_DISK_2.parse::<Disk>().unwrap();
        disk.frag_compact();
        assert_eq!(disk.checksum(), 1928);

        let mut disk = START_DISK_2.parse::<Disk>().unwrap();
        disk.quick_defrag();
        println!("{disk}");
        assert_eq!(disk.checksum(), 2858);
    }

    #[test]
    pub fn test_quick_defrag() {
        let mut disk = START_DISK_2.parse::<Disk>().unwrap();
        disk.quick_defrag();
        assert_eq!(format!("{}", disk), COMPACTED_DISK_LAYOUT_3);
    }
}
