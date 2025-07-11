use std::fs;

use crate::seedlocmap::seedlocmap::SeedLocMap;

mod seedlocmap;

fn main() {
    let seed_loc_map = SeedLocMap::new(fs::File::open("input.txt").unwrap(), false);
    println!("{:?}", seed_loc_map);
    println!("{}", seed_loc_map.lowest_seed_loc());
}
