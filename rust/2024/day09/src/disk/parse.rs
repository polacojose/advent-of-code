use std::{io, str::FromStr};

use crate::disk::disk::{Disk, DiskEntry, DiskEntryKind};

impl FromStr for Disk {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let disk_entries = s
            .trim()
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let length = c
                    .to_string()
                    .parse::<usize>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                let kind = if i % 2 == 0 {
                    DiskEntryKind::Data(i / 2)
                } else {
                    DiskEntryKind::Empty
                };
                Ok(DiskEntry { length, kind })
            })
            .collect::<Result<Vec<_>, Self::Err>>()?;

        Ok(Self { disk_entries })
    }
}
