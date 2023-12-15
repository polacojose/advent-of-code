use std::{collections::HashMap, str::FromStr, usize};

#[derive(Debug, PartialEq, Eq)]
pub struct Draw {
    colors: HashMap<String, usize>,
}

impl Draw {
    #[cfg(test)]
    fn new(sets: HashMap<String, usize>) -> Self {
        Self { colors: sets }
    }
}

#[derive(Debug)]
pub struct UnableToParseDraw;
impl FromStr for Draw {
    type Err = UnableToParseDraw;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color_set = s
            .split(",")
            .map(|x| x.trim())
            .map(|x| x.split_once(" ").unwrap())
            .collect::<Vec<_>>();

        let mut colors: HashMap<String, usize> = HashMap::new();
        for (n_str, color) in color_set {
            let n = n_str.parse::<usize>().unwrap();
            colors
                .entry(color.trim().to_string())
                .and_modify(|x| *x += n)
                .or_insert(n);
        }

        Ok(Self { colors })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GameResult {
    pub id: usize,
    draws: Vec<Draw>,
}

impl GameResult {
    #[cfg(test)]
    fn new(id: usize, draws: Vec<Draw>) -> Self {
        Self { id, draws }
    }

    pub fn valid(&self, components: &HashMap<String, usize>) -> bool {
        for d in &self.draws {
            for (k, v) in &d.colors {
                if let Some(value) = components.get(k) {
                    if v > value {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn minimum_component_power(&self) -> usize {
        let mut minimums: HashMap<String, usize> = HashMap::new();

        for d in &self.draws {
            for (k, v) in &d.colors {
                minimums
                    .entry(k.to_owned())
                    .and_modify(|x| *x = (*x).max(*v))
                    .or_insert(*v);
            }
        }

        return minimums
            .into_iter()
            .map(|x| x.1)
            .reduce(|a, b| a * b)
            .unwrap();
    }
}

#[derive(Debug)]
pub struct UnableToParseGame;
impl FromStr for GameResult {
    type Err = UnableToParseGame;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_string, remaining) = s.split_once(":").unwrap();
        let id = id_string
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let draws = remaining
            .trim()
            .split(";")
            .map(|x| x.parse::<Draw>().unwrap())
            .collect();

        Ok(Self { id, draws })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn can_parse_game_result() {
        let game = "Game 10: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse::<GameResult>()
            .unwrap();
        let mut draws = Vec::new();

        let mut set = HashMap::new();
        set.insert("blue".to_owned(), 3);
        set.insert("red".to_owned(), 4);
        let draw = Draw::new(set);
        draws.push(draw);

        let mut set = HashMap::new();
        set.insert("red".to_owned(), 1);
        set.insert("green".to_owned(), 2);
        set.insert("blue".to_owned(), 6);
        let draw = Draw::new(set);
        draws.push(draw);

        let mut set = HashMap::new();
        set.insert("green".to_owned(), 2);
        let draw = Draw::new(set);
        draws.push(draw);

        let expected_game = GameResult::new(10, draws);
        assert_eq!(game, expected_game);
    }

    #[test]
    fn can_validate_game_result() {
        let mut components = HashMap::new();
        components.insert("red".to_owned(), 12);
        components.insert("green".to_owned(), 13);
        components.insert("blue".to_owned(), 14);

        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse::<GameResult>()
            .unwrap();
        assert!(game.valid(&components) == true);

        let game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
            .parse::<GameResult>()
            .unwrap();
        assert!(game.valid(&components) == true);

        let game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .parse::<GameResult>()
            .unwrap();
        assert!(game.valid(&components) == true);

        let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            .parse::<GameResult>()
            .unwrap();
        assert!(game.valid(&components) == false);

        let game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            .parse::<GameResult>()
            .unwrap();
        assert!(game.valid(&components) == false);
    }

    #[test]
    fn can_add_valid() {
        let game_results = fs::read_to_string("test-input.txt")
            .unwrap()
            .lines()
            .map(|s| s.parse::<GameResult>().unwrap())
            .collect::<Vec<_>>();

        let mut components = HashMap::new();
        components.insert("red".to_owned(), 12);
        components.insert("green".to_owned(), 13);
        components.insert("blue".to_owned(), 14);

        let mut id_total = 0;
        for g in game_results {
            if g.valid(&components) {
                id_total += g.id;
            }
        }

        assert_eq!(id_total, 8);
    }

    #[test]
    fn can_get_minimum_power() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse::<GameResult>()
            .unwrap();
        assert_eq!(game.minimum_component_power(), 48);

        let game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
            .parse::<GameResult>()
            .unwrap();
        assert_eq!(game.minimum_component_power(), 12);

        let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            .parse::<GameResult>()
            .unwrap();
        assert_eq!(game.minimum_component_power(), 1560);

        let game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            .parse::<GameResult>()
            .unwrap();
        assert_eq!(game.minimum_component_power(), 630);

        let game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .parse::<GameResult>()
            .unwrap();
        assert_eq!(game.minimum_component_power(), 36);
    }
}
