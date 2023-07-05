use std::sync::atomic::{AtomicU32, Ordering};

use part01::*;

static MIN_MANA: AtomicU32 = AtomicU32::new(u32::MAX);

fn main() {
    let player_stats = PLAYER_STARTING_STATS.clone();
    let boss_stats = BOSS_STARTING_STATS.clone();

    let mut available_spells = SPELLS
        .clone()
        .into_iter()
        .map(|x| Some(x))
        .collect::<Vec<_>>();
    available_spells.insert(0, None);

    for spell in SPELLS.clone().into_iter().map(|x| x.1).collect::<Vec<_>>() {
        let player_stats = player_stats.clone();
        let boss_stats = boss_stats.clone();
        find_minimum_spells_to_win(&player_stats, &boss_stats, &vec![spell.clone()]);
        println!("Min mana: {}", MIN_MANA.load(Ordering::Relaxed));
    }
}

fn find_minimum_spells_to_win(
    player_stats: &Stats,
    boss_stats: &Stats,
    spell_list: &Vec<SpellType>,
) {
    let available_spells = SPELLS.clone().into_iter().map(|x| x.1).collect::<Vec<_>>();

    let spell_list = spell_list.clone();
    let mut spell_index = 0;
    while spell_index < available_spells.len() {
        let mut spell_list = spell_list.clone();
        let spell = available_spells[spell_index].clone();
        spell_list.push(spell);

        if total_spell_list_cost(&spell_list) > MIN_MANA.load(Ordering::Relaxed) {
            spell_index += 1;
            continue;
        }

        let result = fight(&player_stats, &boss_stats, &spell_list);
        match result {
            Ok(used_mana) => loop {
                let old = MIN_MANA.load(Ordering::Relaxed);
                if used_mana < old {
                    if let Ok(_) = MIN_MANA.compare_exchange(
                        old,
                        used_mana,
                        Ordering::Acquire,
                        Ordering::Acquire,
                    ) {
                        println!("New min mana: {}", MIN_MANA.load(Ordering::Relaxed));
                        break;
                    }
                } else {
                    break;
                }
            },
            Err(error) => match error {
                Error::OutOfSpells => {
                    let player_stats = player_stats.clone();
                    let boss_stats = boss_stats.clone();
                    find_minimum_spells_to_win(&player_stats, &boss_stats, &spell_list.clone());
                }
                _ => {
                    //println!("{:?}", error);
                }
            },
        }

        spell_index += 1;
    }
}
