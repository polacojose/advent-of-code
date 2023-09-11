use std::{collections::VecDeque, sync::RwLock, thread::sleep, time::Duration};

use lazy_static::lazy_static;

const DEST_NUM_PRESENTS: u32 = 33100000;

lazy_static! {
    static ref FACTORS: RwLock<VecDeque<u32>> = {
        let mut v = VecDeque::new();
        v.push_back(2);
        v.push_back(1);
        RwLock::new(v)
    };
}

fn main() {
    for i in 1.. {
        let factors = find_factors(i);

        let presents = factors.iter().map(|&x| x * 10).sum::<u32>();

        if i % 1000 == 0 {
            println!("House {} = {}", i, presents);
        }

        if presents >= DEST_NUM_PRESENTS {
            println!("House {} = {}", i, presents);
            break;
        }
    }
}

fn find_factors(number: u32) -> Vec<u32> {
    let mut factors = vec![number];

    //println!("here");
    let mut truncate = FACTORS.read().unwrap().len();
    for (i, factor) in FACTORS.read().unwrap().iter().enumerate() {
        if factor * 10 < number {
            truncate = i;
            break;
        }

        //   print!("{factor}");

        if number % factor == 0 {
            factors.push(number / factor);
        }
    }

    //println!("{number} | {truncate}, {:?}", *FACTORS.read().unwrap());
    FACTORS.write().unwrap().truncate(truncate);

    //sleep(Duration::from_secs(1));

    if number > *FACTORS.read().unwrap().front().unwrap() {
        FACTORS.write().unwrap().push_front(number);
    }

    factors.dedup();
    factors
}
