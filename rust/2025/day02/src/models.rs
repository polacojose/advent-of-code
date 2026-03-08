use std::{error::Error, fmt::Display, str::FromStr};

use crate::digit::digits;

#[derive(Debug, Clone)]
pub struct IDRange {
    pub start: u64,
    pub end: u64,
}

impl IDRange {
    pub fn contains(&self, n: u64) -> Result<bool, OutOfRange> {
        if n < self.start {
            return Err(OutOfRange::Before);
        } else if self.end < n {
            return Err(OutOfRange::After);
        }
        Ok(true)
    }
}

impl FromStr for IDRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s_str, e_str) = s.split_once("-").ok_or("Invalid String")?;
        let start = s_str.parse().map_err(|e| format!("Invalid start: {e}"))?;
        let end = e_str.parse().map_err(|e| format!("Invalid end: {e}"))?;

        // Current process can only handle a range with one digit difference
        assert!(digits(end) - digits(start) <= 1);

        Ok(Self { start, end })
    }
}

#[derive(Debug)]
pub struct IDRangeOldScanner {
    pub start: u64,
    pub end: u64,
}

impl FromStr for IDRangeOldScanner {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s_str, e_str) = s.split_once("-").ok_or("Invalid String")?;

        Ok(Self {
            start: s_str.parse().map_err(|e| format!("Invalid start: {e}"))?,
            end: e_str.parse().map_err(|e| format!("Invalid end: {e}"))?,
        })
    }
}

#[derive(Debug)]
pub enum OutOfRange {
    Before,
    After,
}

impl Display for OutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for OutOfRange {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
