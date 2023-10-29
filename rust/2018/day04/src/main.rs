use std::fs;

use chrono::Timelike;
use guard::duty_record::DutyRecord;

mod guard;

fn main() {
    let duty_record = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<DutyRecord>()
        .unwrap();
    part1(&duty_record);
    part2(&duty_record);
}

fn part1(duty_record: &DutyRecord) {
    let guard_most_asleep = duty_record.guard_most_total_sleep();
    let most_asleep_segment = duty_record.guard_time_segment_most_asleep(guard_most_asleep);
    println!(
        "Part1: {} * {} = {}",
        guard_most_asleep,
        most_asleep_segment.0.minute(),
        guard_most_asleep * most_asleep_segment.0.minute() as usize
    );
}

fn part2(duty_record: &DutyRecord) {
    let guard_most_asleep = duty_record.guard_most_common_sleep_segment();
    let most_asleep_segment = duty_record.guard_time_segment_most_asleep(guard_most_asleep);
    println!(
        "Part2: {} * {} = {}",
        guard_most_asleep,
        most_asleep_segment.0.minute(),
        guard_most_asleep * most_asleep_segment.0.minute() as usize
    );
}
