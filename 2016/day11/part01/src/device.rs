use std::{fmt::Display, str::FromStr, usize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceType {
    Generator,
    Microchip,
}

#[derive(Debug)]
pub struct UnableToParse;
impl FromStr for DeviceType {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "microchip" => Ok(DeviceType::Microchip),
            "generator" => Ok(DeviceType::Generator),
            _ => Err(UnableToParse),
        }
    }
}

#[derive(Debug, Clone, Eq, Hash)]
pub struct Device {
    pub name: String,
    pub r_type: DeviceType,
    pub position_floor: usize,
    pub position_horizontal: usize,
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r_type == other.r_type
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_output = format!(
            "{}",
            match self.r_type {
                DeviceType::Generator => {
                    "G"
                }
                DeviceType::Microchip => {
                    "M"
                }
            }
        );
        let prefix = self.name[0..=0].to_uppercase();
        write!(f, "{}", format!("{}{}", prefix, type_output))
    }
}
