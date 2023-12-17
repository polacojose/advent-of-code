use std::{collections::VecDeque, str::FromStr};

#[derive(Clone)]
pub struct Card {
    id: u32,
    matches: u32,
}

#[derive(Debug)]
pub struct UnableToParseCardError;
impl FromStr for Card {
    type Err = UnableToParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_group_str, remaining) = s.split_once(':').ok_or(UnableToParseCardError)?;
        let id = id_group_str
            .split_whitespace()
            .last()
            .ok_or(UnableToParseCardError)?
            .parse()
            .map_err(|_| UnableToParseCardError)?;

        let (winning_numbers_str, card_numbers_str) =
            remaining.split_once('|').ok_or(UnableToParseCardError)?;

        let mut winning_numbers = winning_numbers_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|_| UnableToParseCardError)?;
        winning_numbers.sort();

        let mut card_numbers = card_numbers_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|_| UnableToParseCardError)?;
        card_numbers.sort();

        Ok(Card {
            id,
            matches: Self::count_matches(winning_numbers, card_numbers),
        })
    }
}

impl Card {
    pub fn points(&self) -> usize {
        if self.matches > 0 {
            return (2 as usize).pow(self.matches - 1);
        } else {
            return 0;
        }
    }

    fn count_matches(winning_numbers: Vec<i32>, card_numbers: Vec<i32>) -> u32 {
        let mut matches = 0;
        for w in winning_numbers {
            if card_numbers.binary_search(&w).is_ok() {
                matches += 1;
            }
        }
        return matches;
    }
}

pub struct CardTable {
    pub cards: Vec<Card>,
}

impl CardTable {
    pub fn points(&self) -> usize {
        let mut total_points = 0;
        let mut working_set = self.cards.iter().collect::<VecDeque<&Card>>();
        while let Some(card) = working_set.pop_front() {
            let matches = card.matches;
            total_points += 1;
            for i in 0..matches {
                working_set.push_back(self.cards.get((card.id + i) as usize).unwrap());
            }
        }
        return total_points;
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, CardTable};
    use std::str::FromStr;

    macro_rules! test_card_points {
        ($line:literal, $id:literal, $expected_points:literal) => {
            let card = Card::from_str($line).unwrap();
            assert_eq!(card.id, $id);

            let points = card.points();
            assert_eq!(points, $expected_points);
        };
    }

    #[test]
    fn test_points_card() {
        test_card_points!("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 1, 8);
        test_card_points!("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2, 2);
        test_card_points!("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 3, 2);
        test_card_points!("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 4, 1);
        test_card_points!("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 5, 0);
        test_card_points!("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 6, 0);
    }

    #[test]
    fn test_card_table_points() {
        let cards = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .into_iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<Card>, _>>()
        .unwrap();

        let card_table = CardTable { cards };
        assert_eq!(card_table.points(), 30);
    }
}
