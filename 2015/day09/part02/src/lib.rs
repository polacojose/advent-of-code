use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Distance {
    pub source: String,
    pub destination: String,
    pub length: u32,
}

impl FromStr for Distance {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(' ').collect::<Vec<&str>>();
        Ok(Distance {
            source: split[0].to_string(),
            destination: split[2].to_string(),
            length: split[4].parse::<u32>().unwrap(),
        })
    }
}

pub struct DistanceMap {
    set_a: HashMap<String, HashSet<Distance>>,
    set_b: HashMap<String, HashSet<Distance>>,
}

impl DistanceMap {
    pub fn new() -> DistanceMap {
        DistanceMap {
            set_a: HashMap::new(),
            set_b: HashMap::new(),
        }
    }

    pub fn insert(&mut self, distance: Distance) {
        let set_a = self
            .set_a
            .entry(distance.source.to_uppercase().clone())
            .or_insert(HashSet::new());
        let set_b = self
            .set_b
            .entry(distance.destination.to_uppercase().clone())
            .or_insert(HashSet::new());

        set_a.insert(distance.clone());
        set_b.insert(distance.clone());
    }

    pub fn get(&self, key: impl AsRef<str>) -> HashMap<String, u32> {
        let key = key.as_ref().to_uppercase();

        let vec_a = self
            .set_a
            .get(key.as_str())
            .cloned()
            .unwrap_or(HashSet::new());
        let vec_b = self
            .set_b
            .get(key.as_str())
            .cloned()
            .unwrap_or(HashSet::new());

        let mut map = HashMap::new();

        for distance in vec_a.iter() {
            map.insert(distance.destination.clone(), distance.length);
        }

        for distance in vec_b.iter() {
            map.insert(distance.source.clone(), distance.length);
        }

        map
    }

    pub fn get_sets(
        &self,
    ) -> (
        &HashMap<String, HashSet<Distance>>,
        &HashMap<String, HashSet<Distance>>,
    ) {
        (&self.set_a, &self.set_b)
    }
}
