use part01::{
    astar::{find_path, AStarVector},
    INITIAL_HASH,
};

fn main() {
    let start = AStarVector {
        door_hash: INITIAL_HASH.to_string(),
        x: 0,
        y: 0,
    };
    let end = AStarVector {
        door_hash: INITIAL_HASH.to_string(),
        x: 3,
        y: 3,
    };

    let hash = find_path(start, end.clone());

    println!("Hash: {}", hash);
    println!("Hash Length: {}", hash.len());
}
