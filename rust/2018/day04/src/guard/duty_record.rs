use std::{collections::HashMap, ops::Add, str::FromStr, time::Duration};

use chrono::NaiveTime;

use super::duty_record_entry::{DutyRecordEntry, EntryType};

pub struct DutyRecord {
    guard_to_entries: HashMap<usize, Vec<DutyRecordEntry>>,
}

#[derive(Debug)]
pub struct UnableToParseDutyRecord;
impl FromStr for DutyRecord {
    type Err = UnableToParseDutyRecord;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = s
            .lines()
            .map(|s| s.parse::<DutyRecordEntry>().unwrap())
            .collect::<Vec<_>>();
        entries.sort_unstable_by_key(|e| e.time);

        let mut guard_to_entries: HashMap<usize, Vec<DutyRecordEntry>> = HashMap::new();

        let mut current_id = 0;
        for e in entries {
            match e.d_type {
                EntryType::ShiftBegin { id } => {
                    current_id = id;
                }
                _ => {
                    guard_to_entries
                        .entry(current_id)
                        .and_modify(|x| (*x).push(e.clone()))
                        .or_insert(vec![e]);
                }
            }
        }
        Ok(DutyRecord { guard_to_entries })
    }
}

impl DutyRecord {
    fn timespan_to_minute_segments(from: NaiveTime, to: NaiveTime) -> Vec<NaiveTime> {
        let mut segments = vec![from];
        let duration_minutes = (to - from).num_minutes();
        for i in 1..duration_minutes {
            segments.push(from.add(Duration::from_secs(60 * i as u64)))
        }
        segments
    }
}

impl DutyRecord {
    pub fn guard_time_segment_most_asleep(&self, id: usize) -> (NaiveTime, u64) {
        let mut segment_map: HashMap<NaiveTime, u64> = HashMap::new();
        let sleep_durations = self.guard_sleep_durations(id);
        for sd in sleep_durations {
            let minute_segments = DutyRecord::timespan_to_minute_segments(sd.0, sd.1);
            for ms in minute_segments {
                segment_map.entry(ms).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        segment_map.into_iter().max_by_key(|x| x.1).unwrap()
    }

    pub fn guard_most_total_sleep(&self) -> usize {
        *self
            .guard_to_entries
            .iter()
            .map(|(k, _)| (k, self.guard_total_sleep_minutes(*k)))
            .max_by_key(|(_, v)| *v)
            .unwrap()
            .0
    }

    pub fn guard_most_common_sleep_segment(&self) -> usize {
        *self
            .guard_to_entries
            .iter()
            .map(|(k, _)| (k, self.guard_time_segment_most_asleep(*k)))
            .max_by_key(|(_, v)| v.1)
            .unwrap()
            .0
    }

    fn guard_total_sleep_minutes(&self, id: usize) -> i64 {
        let mut total_minutes = 0;
        let sleep_durations = self.guard_sleep_durations(id);
        for sd in sleep_durations {
            total_minutes += (sd.1 - sd.0).num_minutes();
        }
        total_minutes
    }

    fn guard_sleep_durations(&self, id: usize) -> Vec<(NaiveTime, NaiveTime)> {
        let guard_record_entries = self.guard_to_entries.get(&id).unwrap();
        let mut sleep_durations = Vec::new();
        for i in 1..guard_record_entries.len() {
            match guard_record_entries[i - 1].d_type {
                EntryType::FallsAsleep => {
                    let from = guard_record_entries[i - 1].time;
                    let to = guard_record_entries[i].time;
                    sleep_durations.push((from.time(), to.time()));
                }
                _ => {}
            }
        }
        sleep_durations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn test_duty_record() -> Option<DutyRecord> {
        fs::read_to_string("test-input.txt").ok()?.parse().ok()
    }

    #[test]
    fn can_get_minute_segments() {
        let expected = vec![
            NaiveTime::parse_from_str("00:20", "%H:%M").unwrap(),
            NaiveTime::parse_from_str("00:21", "%H:%M").unwrap(),
            NaiveTime::parse_from_str("00:22", "%H:%M").unwrap(),
            NaiveTime::parse_from_str("00:23", "%H:%M").unwrap(),
            NaiveTime::parse_from_str("00:24", "%H:%M").unwrap(),
        ];
        let from = NaiveTime::parse_from_str("00:20", "%H:%M").unwrap();
        let to = NaiveTime::parse_from_str("00:25", "%H:%M").unwrap();
        let segments = DutyRecord::timespan_to_minute_segments(from, to);
        assert!(segments == expected);
    }

    #[test]
    fn can_get_guard_sleep_length() {
        let duty_record = test_duty_record().unwrap();
        let guard_total_sleep_minutes = duty_record.guard_total_sleep_minutes(10);
        println!("{:?}", guard_total_sleep_minutes);
        assert!(guard_total_sleep_minutes == 50);
    }

    #[test]
    fn can_get_common_sleep_time() {
        let duty_record = test_duty_record().unwrap();
        let common_sleep_time = duty_record.guard_time_segment_most_asleep(10);
        assert!(common_sleep_time.0 == NaiveTime::parse_from_str("00:24", "%H:%M").unwrap());
    }

    #[test]
    fn can_get_guard_most_total_sleep() {
        let duty_record = test_duty_record().unwrap();
        assert!(duty_record.guard_most_total_sleep() == 10);
    }

    #[test]
    fn can_get_guard_most_common_sleep_segment() {
        let duty_record = test_duty_record().unwrap();
        assert!(duty_record.guard_most_common_sleep_segment() == 99);
    }
}
