use std::sync::RwLock;

use lazy_static::lazy_static;

const DEST_NUM_PRESENTS: u32 = 33100000;

lazy_static! {
    static ref FACTORS: RwLock<Vec<u32>> = RwLock::new(vec![2]);
}

fn main() {
    for i in 1.. {
        let factors = find_factors(i);

        //println!("{:?}", factors);

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
