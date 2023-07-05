use std::fs;

use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    name: String,
    cost: u32,
    damage: u32,
    armor: u32,
}

#[derive(Clone)]
struct Stats {
    hp: u32,
    damage: u32,
    armor: u32,
}

impl From<&Inventory> for Stats {
    fn from(inventory: &Inventory) -> Self {
        let mut total_damage = 0;
        let mut total_armor = 0;

        if let Some(weapon) = &inventory.weapon {
            total_damage += weapon.damage;
            total_armor += weapon.armor;
        }

        if let Some(armor) = &inventory.armor {
            total_damage += armor.damage;
            total_armor += armor.armor;
        }

        if let Some(ring) = &inventory.left_ring {
            total_damage += ring.damage;
            total_armor += ring.armor;
        }

        if let Some(ring) = &inventory.right_ring {
            total_damage += ring.damage;
            total_armor += ring.armor;
        }

        Self {
            hp: 100,
            damage: total_damage,
            armor: total_armor,
        }
    }
}

#[derive(Debug)]
struct Inventory {
    weapon: Option<Item>,
    armor: Option<Item>,
    left_ring: Option<Item>,
    right_ring: Option<Item>,
}

impl Inventory {
    fn total_cost(&self) -> u32 {
        self.weapon.as_ref().map(|w| w.cost).unwrap_or(0)
            + self.armor.as_ref().map(|a| a.cost).unwrap_or(0)
            + self.left_ring.as_ref().map(|r| r.cost).unwrap_or(0)
            + self.right_ring.as_ref().map(|r| r.cost).unwrap_or(0)
    }
}

fn get_item_from_line(line: &str) -> Item {
    let parts = line
        .split("  ")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    let item = Item {
        name: parts[0].to_string(),
        cost: parts[1].parse::<u32>().unwrap(),
        damage: parts[2].parse::<u32>().unwrap(),
        armor: parts[3].parse::<u32>().unwrap(),
    };

    item
}

lazy_static! {
    static ref WEAPONS: Vec<Option<Item>> = {
        let mut items = vec![];
        items.append(&mut fs::read_to_string("assets/weapons.txt")
            .unwrap()
            .lines()
            .map(|l|Some(get_item_from_line(l)))
            .collect());
        items
    };
    static ref RINGS: Vec<Option<Item>> = {
        let mut items = vec![None];
        items.append(&mut fs::read_to_string("assets/rings.txt")
            .unwrap()
            .lines()
            .map(|l|Some(get_item_from_line(l)))
            .collect());
        items
    };
    static ref ARMOR: Vec<Option<Item>> = {
        let mut items = vec![None];
        items.append(&mut fs::read_to_string("assets/armor.txt")
            .unwrap()
            .lines()
            .map(|l|Some(get_item_from_line(l)))
            .collect());
        items
    };

    #[derive(Clone)]
    static ref BOSS_STATS: Stats = {
        Stats {
            hp: 100,
            damage: 8,
            armor: 2,
        }
    };
}

fn main() {
    let mut max_cost = 0;
    for weapon in WEAPONS.iter() {
        for armor in ARMOR.iter() {
            for left_ring in RINGS.iter() {
                for right_ring in RINGS.iter() {
                    if let (Some(left_r), Some(right_r)) = (left_ring, right_ring) {
                        if right_r == left_r {
                            continue;
                        }
                    }
                    let inventory = Inventory {
                        weapon: weapon.clone(),
                        armor: armor.clone(),
                        left_ring: left_ring.clone(),
                        right_ring: right_ring.clone(),
                    };
                    if !fight_with_inventory(&inventory) {
                        if inventory.total_cost() > max_cost {
                            max_cost = inventory.total_cost();
                            println!("{:?}", inventory);
                            println!("{}", inventory.total_cost());
                        }
                    }
                }
            }
        }
    }
}

fn fight_with_inventory(inventory: &Inventory) -> bool {
    let player_stats = Stats::from(inventory);
    let boss_stats = &*BOSS_STATS;
    fight_stats(&player_stats, &boss_stats)
}

fn fight_stats(player_stats: &Stats, boss_stats: &Stats) -> bool {
    let mut player_stats = player_stats.clone();
    let mut boss_stats = boss_stats.clone();
    while player_stats.hp > 0 && boss_stats.hp > 0 {
        let damage = player_stats.damage.saturating_sub(boss_stats.armor).max(1);
        boss_stats.hp = boss_stats.hp.saturating_sub(damage);
        // println!(
        //     "The player deals {}-{} = {} damage; the boss goes down to {} hit points.",
        //     player_stats.damage, boss_stats.armor, damage, boss_stats.hp
        // );

        if boss_stats.hp <= 0 {
            break;
        }

        let damage = boss_stats.damage.saturating_sub(player_stats.armor).max(1);
        player_stats.hp = player_stats.hp.saturating_sub(damage);
        // println!(
        //     "The boss deals {}-{} = {} damage; the player goes down to {} hit points.",
        //     boss_stats.damage, player_stats.armor, damage, player_stats.hp
        // );
    }

    player_stats.hp > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fight() {
        let player_stats = Stats {
            hp: 8,
            damage: 5,
            armor: 5,
        };

        let boss_stats = Stats {
            hp: 12,
            damage: 7,
            armor: 2,
        };

        assert!(fight_stats(&player_stats, &boss_stats));
    }

    #[test]
    fn test_inventory_total() {
        let inventory = test_inventory();
        assert_eq!(inventory.total_cost(), 1000);
    }

    #[test]
    fn test_stats() {
        let stats = Stats::from(&test_inventory());
        assert_eq!(stats.damage, 16);
        assert_eq!(stats.armor, 20);
    }

    fn test_inventory() -> Inventory {
        Inventory {
            weapon: Some(Item {
                name: "Item".to_string(),
                cost: 100,
                damage: 1,
                armor: 2,
            }),
            armor: Some(Item {
                name: "Item".to_string(),
                cost: 200,
                damage: 3,
                armor: 4,
            }),
            left_ring: Some(Item {
                name: "Item".to_string(),
                cost: 300,
                damage: 5,
                armor: 6,
            }),
            right_ring: Some(Item {
                name: "Item".to_string(),
                cost: 400,
                damage: 7,
                armor: 8,
            }),
        }
    }
}
