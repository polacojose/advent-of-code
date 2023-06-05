use std::fs;

use lazy_static::lazy_static;

#[derive(Debug, Clone)]
struct RacingDeer {
    name: String,
    racing_rate: u32,
    race_period: u32,
    rest_period: u32,
    position: u32,
    race_seconds: u32,
}

impl RacingDeer {
    fn set_race_seconds(&mut self, seconds: u32) {
        self.race_seconds = seconds;

        let mut position = (self.racing_rate * self.race_period)
            * (self.race_seconds / (self.rest_period + self.race_period));
        let remainder = self.race_seconds % (self.rest_period + self.race_period);

        position += self.racing_rate * remainder.min(self.race_period);

        self.position = position
    }

    fn get_position(&self) -> u32 {
        self.position
    }
}

lazy_static! {
    static ref RACING_DEER: Vec<RacingDeer> = {
        let mut racing_deer = Vec::new();

        let file_string = fs::read_to_string("input.txt").unwrap();
        for line in file_string.lines() {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();

            racing_deer.push(RacingDeer {
                name: tokens[0].to_string(),
                racing_rate: tokens[3].parse().unwrap(),
                race_period: tokens[6].parse().unwrap(),
                rest_period: tokens[13].parse().unwrap(),
                position: 0,
                race_seconds: 0,
            });
        }

        racing_deer
    };
}

fn main() {
    let mut all_racing_deer = RACING_DEER.clone();
    for racing_deer in all_racing_deer.iter_mut() {
        racing_deer.set_race_seconds(2503);
    }

    all_racing_deer.sort_by(|a, b| a.position.cmp(&b.position));

    for racing_deer in all_racing_deer.iter_mut() {
        println!("{:?}", racing_deer);
    }
}
