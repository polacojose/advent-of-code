use std::{collections::HashSet, fs, hash::Hash};

use lazy_static::lazy_static;
use regex::Regex;

use crate::device::{Device, DeviceType};
const FLOOR_HEIGHT: usize = 4;

lazy_static! {
    pub static ref INITIAL_GAME_STATE: GameState = {
        let file_string = fs::read_to_string("input.txt").unwrap();
        let re = Regex::new(r"(x?) a (?P<name>[a-z-]+) (?P<device_type>\w+)").unwrap();

        let mut devices = Vec::new();
        let mut position_horizontal = 0;
        for (floor, line) in file_string.lines().enumerate() {
            let captures = re.captures_iter(line);
            for capture in captures {
                devices.push(Device {
                    position_floor: floor,
                    position_horizontal,
                    name: capture
                        .name("name")
                        .unwrap()
                        .as_str()
                        .split("-")
                        .collect::<Vec<_>>()
                        .first()
                        .unwrap()
                        .to_string(),
                    r_type: capture
                        .name("device_type")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                });
                position_horizontal += 1;
            }
        }
        GameState {
            previous_states: HashSet::new(),
            elevator_position: 0,
            devices: devices,
        }
    };
}

#[derive(Eq)]
pub struct GameState {
    previous_states: HashSet<GameState>,
    elevator_position: usize,
    devices: Vec<Device>,
}

impl Clone for GameState {
    fn clone(&self) -> Self {
        Self {
            previous_states: Default::default(),
            elevator_position: self.elevator_position.clone(),
            devices: self.devices.clone(),
        }
    }
}

impl Hash for GameState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.elevator_position.hash(state);
        self.devices.hash(state);
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.elevator_position == other.elevator_position && self.devices == other.devices
    }
}

impl GameState {
    pub fn is_victory_state(&self) -> bool {
        self.devices.iter().map(|d| d.position_floor).min().unwrap() == FLOOR_HEIGHT - 1
    }

    pub fn get_floor_device_combinations(&self, floor: usize) -> Vec<Vec<Device>> {
        let mut combinations = Vec::new();
        let floor_devices = self
            .devices
            .iter()
            .filter(|x| x.position_floor == floor)
            .collect::<Vec<_>>();

        for device_index in 0..floor_devices.len() {
            combinations.push(vec![floor_devices[device_index].clone()]);
            for second_device_index in (device_index + 1)..floor_devices.len() {
                combinations.push(vec![
                    floor_devices[device_index].clone(),
                    floor_devices[second_device_index].clone(),
                ])
            }
        }

        combinations
    }

    pub fn get_next_valid_states(&self) -> Vec<GameState> {
        let mut gamestates = Vec::new();

        if self.elevator_position < FLOOR_HEIGHT - 1 {
            let new_elevator_position = self.elevator_position + 1;
            let new_game_states_devices =
                self.get_floor_device_combinations(self.elevator_position);
            for new_game_state_devices in new_game_states_devices {
                let mut new_game_state = self.clone();
                new_game_state.elevator_position = new_elevator_position;
                for device in new_game_state.devices.iter_mut() {
                    if new_game_state_devices.contains(&device) {
                        device.position_floor = new_elevator_position;
                    }
                }

                if !self.previous_states.contains(&new_game_state)
                    && new_game_state.valid_game_state()
                {
                    new_game_state.previous_states = self.previous_states.clone();
                    new_game_state.previous_states.insert(self.clone());
                    gamestates.push(new_game_state);
                }
            }
        }

        if self.elevator_position > 0 {
            let new_elevator_position = self.elevator_position - 1;
            let new_game_states_devices =
                self.get_floor_device_combinations(self.elevator_position);
            for new_game_state_devices in new_game_states_devices {
                let mut new_game_state = self.clone();
                new_game_state.elevator_position = new_elevator_position;
                for device in new_game_state.devices.iter_mut() {
                    if new_game_state_devices.contains(&device) {
                        device.position_floor = new_elevator_position;
                    }
                }

                if !self.previous_states.contains(&new_game_state)
                    && new_game_state.valid_game_state()
                {
                    new_game_state.previous_states = self.previous_states.clone();
                    new_game_state.previous_states.insert(self.clone());
                    gamestates.push(new_game_state);
                }
            }
        }

        gamestates
    }

    fn valid_game_state(&self) -> bool {
        for floor in 0..FLOOR_HEIGHT {
            if !self.valid_floor(floor) {
                return false;
            }
        }
        true
    }

    fn valid_floor(&self, floor: usize) -> bool {
        let microchips = self
            .devices
            .iter()
            .filter(|x| x.r_type == DeviceType::Microchip && x.position_floor == floor)
            .cloned()
            .collect::<Vec<Device>>();
        for microchip in microchips {
            let is_with_paired_generator = self
                .devices
                .iter()
                .find(|x| {
                    x.position_floor == floor
                        && x.name == microchip.name
                        && x.r_type == DeviceType::Generator
                })
                .is_some();
            let is_with_other_generator = self
                .devices
                .iter()
                .find(|x| {
                    x.position_floor == floor
                        && x.name != microchip.name
                        && x.r_type == DeviceType::Generator
                })
                .is_some();
            if !is_with_paired_generator && is_with_other_generator {
                return false;
            }
        }

        true
    }

    pub fn get_floor_width(&self) -> usize {
        self.devices.len()
    }

    pub fn print_floors(&self) {
        let floor_width = self.get_floor_width();

        for y in (0..FLOOR_HEIGHT).rev() {
            print!("F{}  ", y + 1);
            print!(
                "{:2} ",
                if y == self.elevator_position.into() {
                    "E"
                } else {
                    "."
                }
            );

            let floor_devices = self
                .devices
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .filter(|x| x.position_floor == y)
                .collect::<Vec<_>>();

            for x in 0..floor_width {
                let floor_item_display = if let Some(device) =
                    floor_devices.iter().find(|d| d.position_horizontal == x)
                {
                    format!("{:2}", device)
                } else {
                    ".".to_string()
                };

                print!("{:2} ", floor_item_display);
            }
            println!()
        }
    }
}
