use std::sync::RwLock;

use dashmap::DashMap;
use lazy_static::lazy_static;
use tokio::task::JoinSet;

const DEST_NUM_PRESENTS: u32 = 33100000;

lazy_static! {
    static ref FACTORS: RwLock<Vec<u32>> = RwLock::new(vec![2]);
    static ref HOUSES_MAP: DashMap<u32, u32> = DashMap::new();
    static ref BEST_HOUSE: RwLock<(u32, u32)> = RwLock::new((u32::MAX, u32::MAX));
    static ref DONE: RwLock<bool> = RwLock::new(false);
}

#[tokio::main]
async fn main() {
    let mut join_set = JoinSet::new();
    for i in (1..1000000) {
        if *DONE.read().unwrap() {
            break;
        }
        join_set.spawn(async move {
            let nth = i;
            for i in (i..=(i * 50)).step_by(i) {
                HOUSES_MAP
                    .entry(i as u32)
                    .and_modify(|n| *n += nth as u32 * 11)
                    .or_insert(nth as u32 * 11);

                if let Some(presents) = HOUSES_MAP.get(&(i as u32)) {
                    if *presents >= DEST_NUM_PRESENTS {
                        if BEST_HOUSE.read().unwrap().0 > i as u32 {
                            *BEST_HOUSE.write().unwrap() = (i as u32, *presents);
                            *DONE.write().unwrap() = true;
                            println!(
                                "Best House {} = {}",
                                BEST_HOUSE.read().unwrap().0,
                                BEST_HOUSE.read().unwrap().1
                            );
                        }
                    }
                }
            }
        });
    }

    while let Some(join) = join_set.join_next().await {}
}

fn find_factors(number: u32) -> Vec<u32> {
    let mut factors = vec![number];

    for factor in FACTORS.read().unwrap().iter() {
        if factor >= &number {
            break;
        }

        if number % factor == 0 {
            factors.push(number / factor);
        }
    }

    factors.push(1);

    if number > *FACTORS.read().unwrap().last().unwrap() {
        FACTORS.write().unwrap().push(number);
    }
    //assert!(n <= *FACTORS.read().unwrap().last().unwrap(), "{}", n);

    factors.dedup();
    factors
}
