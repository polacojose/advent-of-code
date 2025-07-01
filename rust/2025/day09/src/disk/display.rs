use std::fmt::{Debug, Display};

use crate::disk::disk::{Disk, DiskEntryKind};

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in self.disk_entries.iter() {
            for _ in 0..e.length {
                match e.kind {
                    DiskEntryKind::Data(id) => {
                        write!(f, "{}", id)?;
                    }
                    DiskEntryKind::Empty => {
                        write!(f, ".")?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl Debug for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in self.disk_entries.iter() {
            for _ in 0..e.length {
                match e.kind {
                    DiskEntryKind::Data(id) => {
                        write!(f, "{:4}|", id)?;
                    }
                    DiskEntryKind::Empty => {
                        write!(f, "{:4}|", "")?;
                    }
                }
            }
        }
        Ok(())
    }
}
