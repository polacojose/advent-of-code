use std::str::FromStr;

use chrono::NaiveDateTime;

#[derive(PartialEq, Eq, Debug, Clone)]
pub(super) enum EntryType {
    ShiftBegin { id: usize },
    FallsAsleep,
    WakesUp,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DutyRecordEntry {
    pub(super) time: NaiveDateTime,
    pub(super) d_type: EntryType,
}

#[derive(Debug)]
pub struct UnableToParseDutyRecordEntry;
impl FromStr for DutyRecordEntry {
    type Err = UnableToParseDutyRecordEntry;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let time = NaiveDateTime::parse_from_str(&s[1..17].to_string(), "%Y-%m-%d %H:%M")
            .map_err(|_| UnableToParseDutyRecordEntry)?;

        let parts = s[19..].split_whitespace().collect::<Vec<_>>();
        let d_type = match parts[0] {
            "Guard" => {
                let id = parts[1][1..]
                    .parse()
                    .map_err(|_| UnableToParseDutyRecordEntry)?;
                Ok(EntryType::ShiftBegin { id })
            }
            "falls" => Ok(EntryType::FallsAsleep),
            "wakes" => Ok(EntryType::WakesUp),
            _ => Err(UnableToParseDutyRecordEntry),
        }?;

        Ok(DutyRecordEntry { time, d_type })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        let expected = DutyRecordEntry {
            time: NaiveDateTime::parse_from_str("1518-11-01 00:00", "%Y-%m-%d %H:%M").unwrap(),
            d_type: EntryType::ShiftBegin { id: 10 },
        };
        assert!(
            "[1518-11-01 00:00] Guard #10 begins shift"
                .parse::<DutyRecordEntry>()
                .unwrap()
                == expected
        );

        let expected = DutyRecordEntry {
            time: NaiveDateTime::parse_from_str("1518-11-01 00:05", "%Y-%m-%d %H:%M").unwrap(),
            d_type: EntryType::FallsAsleep,
        };
        assert!(
            "[1518-11-01 00:05] falls asleep"
                .parse::<DutyRecordEntry>()
                .unwrap()
                == expected
        );

        let expected = DutyRecordEntry {
            time: NaiveDateTime::parse_from_str("1518-11-01 00:25", "%Y-%m-%d %H:%M").unwrap(),
            d_type: EntryType::WakesUp,
        };
        assert!(
            "[1518-11-01 00:25] wakes up"
                .parse::<DutyRecordEntry>()
                .unwrap()
                == expected
        );
    }
}
