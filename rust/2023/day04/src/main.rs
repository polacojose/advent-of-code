use std::fs;

use card::Card;

use crate::card::CardTable;

mod card;

fn main() {
    let cards = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse())
        .collect::<Result<Vec<Card>, _>>()
        .unwrap();

    let total_card_points = cards.iter().map(|c| c.points()).sum::<usize>();
    println!("Total points: {}", total_card_points);

    let card_table = CardTable { cards };
    println!("Total card table points: {}", card_table.points());
}
