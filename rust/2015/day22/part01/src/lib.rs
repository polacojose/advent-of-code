use anyhow::Result;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::RwLock};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not enough mana")]
    NotEnoughMana,

    #[error("Spell already active")]
    SpellAlreadyActive,

    #[error("Out of spells")]
    OutOfSpells,

    #[error("Player died")]
    PlayerDied,
}

#[derive(Debug, Clone)]
pub enum SpellType {
    MagicMissle { cost: u32, damage: u32 },
    Drain { cost: u32, damage: u32 },
    Shield { cost: u32, armor: u32, turns: i32 },
    Poison { cost: u32, damage: u32, turns: i32 },
    Recharge { cost: u32, mana: u32, turns: i32 },
}

impl Eq for SpellType {}
impl PartialEq for SpellType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::MagicMissle {
                    cost: l_cost,
                    damage: l_damage,
                },
                Self::MagicMissle {
                    cost: r_cost,
                    damage: r_damage,
                },
            ) => l_cost == r_cost && l_damage == r_damage,
            (
                Self::Drain {
                    cost: l_cost,
                    damage: l_damage,
                },
                Self::Drain {
                    cost: r_cost,
                    damage: r_damage,
                },
            ) => l_cost == r_cost && l_damage == r_damage,
            (
                Self::Shield {
                    cost: l_cost,
                    armor: l_armor,
                    turns: _,
                },
                Self::Shield {
                    cost: r_cost,
                    armor: r_armor,
                    turns: _,
                },
            ) => l_cost == r_cost && l_armor == r_armor,
            (
                Self::Poison {
                    cost: l_cost,
                    damage: l_damage,
                    turns: _,
                },
                Self::Poison {
                    cost: r_cost,
                    damage: r_damage,
                    turns: _,
                },
            ) => l_cost == r_cost && l_damage == r_damage,
            (
                Self::Recharge {
                    cost: l_cost,
                    mana: l_mana,
                    turns: _,
                },
                Self::Recharge {
                    cost: r_cost,
                    mana: r_mana,
                    turns: _,
                },
            ) => l_cost == r_cost && l_mana == r_mana,
            _ => false,
        }
    }
}

impl SpellType {
    pub fn cost(&self) -> u32 {
        match self {
            SpellType::MagicMissle { cost, damage: _ } => *cost,
            SpellType::Drain { cost, damage: _ } => *cost,
            SpellType::Shield {
                cost,
                armor: _,
                turns: _,
            } => *cost,
            SpellType::Poison {
                cost,
                damage: _,
                turns: _,
            } => *cost,
            SpellType::Recharge {
                cost,
                mana: _,
                turns: _,
            } => *cost,
        }
    }

    pub fn immediate(&mut self, source: &mut Stats, target: &mut Stats) -> Result<(), Error> {
        match self {
            SpellType::MagicMissle { cost, damage } => {
                if source.mana < *cost {
                    return Err(Error::NotEnoughMana.into());
                }
                source.mana = source.mana.saturating_sub(*cost);
                target.hp = target.hp.saturating_sub(*damage);
                if *FIGHT_PRINTING.read().unwrap() {
                    println!("Player casts Magic Missle, dealing {} damage.", *damage);
                }
            }
            SpellType::Drain { cost, damage } => {
                if source.mana < *cost {
                    return Err(Error::NotEnoughMana.into());
                }
                source.mana = source.mana.saturating_sub(*cost);
                target.hp = target.hp.saturating_sub(*damage);
                source.hp += *damage;
                if *FIGHT_PRINTING.read().unwrap() {
                    println!(
                        "Player casts Drain, dealing {} damage, and healing {} hit points.",
                        *damage, *damage
                    );
                }
            }
            SpellType::Shield { cost, armor, turns } => {
                if source.mana < *cost {
                    return Err(Error::NotEnoughMana.into());
                }
                source.mana = source.mana.saturating_sub(*cost);
                source.armor = source.armor.saturating_add(*armor);
                *turns = 6;
                if *FIGHT_PRINTING.read().unwrap() {
                    println!("Player casts Shield. Increasing armor by {}.", *armor);
                }
            }
            SpellType::Poison {
                cost,
                damage: _,
                turns,
            } => {
                if source.mana < *cost {
                    return Err(Error::NotEnoughMana.into());
                }
                source.mana = source.mana.saturating_sub(*cost);
                *turns = 6;
                if *FIGHT_PRINTING.read().unwrap() {
                    println!("Player casts Poison.");
                }
            }
            SpellType::Recharge {
                cost,
                mana: _,
                turns,
            } => {
                if source.mana < *cost {
                    return Err(Error::NotEnoughMana.into());
                }
                source.mana = source.mana.saturating_sub(*cost);
                *turns = 5;
                if *FIGHT_PRINTING.read().unwrap() {
                    println!("Player casts Recharge.");
                }
            }
        }
        Ok(())
    }

    pub fn per_turn(&mut self, source: &mut Stats, target: &mut Stats) {
        match self {
            SpellType::Shield {
                cost: _,
                armor,
                turns,
            } => {
                assert!(*turns > 0);
                *turns -= 1;
                if *FIGHT_PRINTING.read().unwrap() {
                    println!("Shield timer is now {}.", *turns);
                }

                if *turns == 0 {
                    if *FIGHT_PRINTING.read().unwrap() {
                        println!("Shield wears off, decreasing armor by {}.", armor);
                    }
                    self.end(source, target);
                }
            }
            SpellType::Poison {
                cost: _,
                damage,
                turns,
            } => {
                assert!(*turns > 0);
                target.hp = target.hp.saturating_sub(*damage);
                *turns -= 1;
                if *FIGHT_PRINTING.read().unwrap() {
                    println!(
                        "Poison deals {} damage; its timer is now {}.",
                        *damage, *turns
                    );
                }

                if *turns == 0 {
                    if *FIGHT_PRINTING.read().unwrap() {
                        println!("Poison wears off.");
                    }
                    self.end(source, target);
                }
            }
            SpellType::Recharge {
                cost: _,
                mana,
                turns,
            } => {
                assert!(*turns > 0);
                source.mana += *mana;
                *turns -= 1;
                if *FIGHT_PRINTING.read().unwrap() {
                    println!(
                        "Recharge restores {} mana; its timer is now {}.",
                        *mana, *turns
                    );
                }

                if *turns == 0 {
                    if *FIGHT_PRINTING.read().unwrap() {
                        println!("Recharge wears off.");
                    }
                    self.end(source, target);
                }
            }
            _ => {}
        }
    }

    pub fn has_turns(&self) -> bool {
        match self {
            SpellType::Shield {
                cost: _,
                armor: _,
                turns,
            }
            | SpellType::Poison {
                cost: _,
                damage: _,
                turns,
            }
            | SpellType::Recharge {
                cost: _,
                mana: _,
                turns,
            } => *turns > 0,
            _ => false,
        }
    }

    pub fn end(&mut self, _source: &mut Stats, target: &mut Stats) {
        match self {
            SpellType::Shield {
                cost: _,
                armor,
                turns,
            } => {
                assert!(*turns == 0);
                target.armor = target.armor.saturating_sub(*armor);
                *turns -= 1;
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
pub struct Stats {
    hp: u32,
    mana: u32,
    armor: u32,
}

pub const BOSS_DAMAGE: u32 = 8;

lazy_static! {
    pub static ref FIGHT_PRINTING: RwLock<bool> = RwLock::new(false);
    pub static ref SPELLS: HashMap<String, SpellType> = {
        let mut spells = HashMap::new();
        spells.insert(
            "magic_missle".to_string(),
            SpellType::MagicMissle {
                cost: 53,
                damage: 4,
            },
        );
        spells.insert(
            "drain".to_string(),
            SpellType::Drain {
                cost: 73,
                damage: 2,
            },
        );
        spells.insert(
            "shield".to_string(),
            SpellType::Shield {
                cost: 113,
                armor: 7,
                turns: 6,
            },
        );
        spells.insert(
            "poison".to_string(),
            SpellType::Poison {
                cost: 173,
                damage: 3,
                turns: 6,
            },
        );
        spells.insert(
            "recharge".to_string(),
            SpellType::Recharge {
                cost: 229,
                mana: 101,
                turns: 5,
            },
        );
        spells
    };
    pub static ref PLAYER_STARTING_STATS: Stats = {
        Stats {
            hp: 50,
            mana: 500,
            armor: 0,
        }
    };
    pub static ref BOSS_STARTING_STATS: Stats = {
        Stats {
            hp: 55,
            mana: 0,
            armor: 0,
        }
    };
}

pub fn total_spell_list_cost(spell_list: &Vec<SpellType>) -> u32 {
    spell_list
        .iter()
        .map(|s| match s {
            SpellType::MagicMissle { cost, damage } => cost,
            SpellType::Drain { cost, damage } => cost,
            SpellType::Shield { cost, armor, turns } => cost,
            SpellType::Poison {
                cost,
                damage,
                turns,
            } => cost,
            SpellType::Recharge { cost, mana, turns } => cost,
        })
        .sum()
}

pub fn fight(
    player_stats: &Stats,
    boss_stats: &Stats,
    spell_list: &Vec<SpellType>,
) -> Result<u32, Error> {
    let mut player_stats = player_stats.clone();
    let mut boss_stats = boss_stats.clone();
    let mut spells = spell_list.clone();

    let mut active_spell_effects: Vec<SpellType> = Vec::new();
    let mut spent_mana = 0;
    while player_stats.hp > 0 && boss_stats.hp > 0 {
        if *FIGHT_PRINTING.read().unwrap() {
            println!();
            println!("-- Player Turn --");
            println!(
                "- Player has {} hit points, {} armor, {} mana",
                player_stats.hp, player_stats.armor, player_stats.mana
            );
            println!("- Boss has {} hit points", boss_stats.hp,);
        }
        process_active_spell_effects(
            &mut active_spell_effects,
            &mut player_stats,
            &mut boss_stats,
        );

        if spells.is_empty() {
            if *FIGHT_PRINTING.read().unwrap() {
                println!("Out of spells! Player loses!");
            }
            return Err(Error::OutOfSpells);
        }

        spent_mana += perform_immediate_spells(
            &mut spells,
            &mut player_stats,
            &mut boss_stats,
            &mut active_spell_effects,
        )?;

        if boss_stats.hp <= 0 {
            break;
        }

        if *FIGHT_PRINTING.read().unwrap() {
            println!();
            println!("-- Boss Turn --");
            println!(
                "- Player has {} hit points, {} armor, {} mana",
                player_stats.hp, player_stats.armor, player_stats.mana
            );
            println!("- Boss has {} hit points", boss_stats.hp);
        }
        process_active_spell_effects(
            &mut active_spell_effects,
            &mut player_stats,
            &mut boss_stats,
        );
        if boss_stats.hp <= 0 {
            if *FIGHT_PRINTING.read().unwrap() {
                println!("==========================");
                println!("This kills the boss, and the player wins.");
            }
            break;
        }

        let damage = BOSS_DAMAGE.saturating_sub(player_stats.armor.max(0));
        if *FIGHT_PRINTING.read().unwrap() {
            if player_stats.armor > 0 {
                println!(
                    "Boss attacks for {} - {} = {} damage!",
                    BOSS_DAMAGE, player_stats.armor, damage
                );
            } else {
                println!("Boss attacks for {} damage!", damage);
            }
        }
        player_stats.hp = player_stats.hp.saturating_sub(damage);
    }

    if player_stats.hp > 0 {
        Ok(spent_mana)
    } else {
        Err(Error::PlayerDied)
    }
}

fn perform_immediate_spells(
    spells: &mut Vec<SpellType>,
    player_stats: &mut Stats,
    boss_stats: &mut Stats,
    active_spell_effects: &mut Vec<SpellType>,
) -> Result<u32, Error> {
    let mut spell = spells.remove(0);

    if active_spell_effects.contains(&spell) {
        return Err(Error::SpellAlreadyActive.into());
    }

    if *FIGHT_PRINTING.read().unwrap() {
        println!("{:?} {:?}", spell, active_spell_effects);
    }

    spell.immediate(player_stats, boss_stats)?;
    if spell.has_turns() {
        active_spell_effects.push(spell.clone());
    }

    Ok(spell.cost())
}

fn process_active_spell_effects(
    active_spell_effects: &mut Vec<SpellType>,
    player_stats: &mut Stats,
    boss_stats: &mut Stats,
) {
    let mut spell_index = 0;
    loop {
        if let Some(ref mut spell) = active_spell_effects.get_mut(spell_index) {
            spell.per_turn(player_stats, boss_stats);
            if !spell.has_turns() {
                active_spell_effects.remove(spell_index);
            } else {
                spell_index += 1;
            }
            continue;
        }
        break;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_fight() {
        let player_stats = Stats {
            hp: 10,
            mana: 250,
            armor: 0,
        };

        let boss_stats = Stats {
            hp: 13,
            mana: 0,
            armor: 0,
        };

        let spells = vec![
            SPELLS.get("poison").unwrap().clone(),
            SPELLS.get("magic_missle").unwrap().clone(),
        ];

        if let Ok(spend_mana) = fight(&player_stats, &boss_stats, &spells) {
            println!("Mana spent: {}", spend_mana);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_demo_fight2() {
        let player_stats = Stats {
            hp: 10,
            mana: 250,
            armor: 0,
        };

        let boss_stats = Stats {
            hp: 14,
            mana: 0,
            armor: 0,
        };

        let spells = vec![
            SPELLS.get("recharge").unwrap().clone(),
            SPELLS.get("shield").unwrap().clone(),
            SPELLS.get("drain").unwrap().clone(),
            SPELLS.get("poison").unwrap().clone(),
            SPELLS.get("magic_missle").unwrap().clone(),
        ];

        if let Ok(spend_mana) = fight(&player_stats, &boss_stats, &spells) {
            println!("Mana spent: {}", spend_mana);
        } else {
            assert!(false);
        }
    }
}
