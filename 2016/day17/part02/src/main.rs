use float_ord::FloatOrd;
use part01::{
    astar::{distance, find_path, AStarVector},
    office::{display_output, display_output_marked, is_wall},
    INITIAL_HASH,
};

fn main() {
    println!("{}", display_output(31, 39));

    let start = AStarVector {
        door_hash: INITIAL_HASH.to_string(),
        f_score: Some(FloatOrd { 0: 0.0 }),
        x: 0,
        y: 0,
    };
    let end = AStarVector {
        door_hash: INITIAL_HASH.to_string(),
        f_score: Some(FloatOrd { 0: 0.0 }),
        x: 3,
        y: 3,
    };

    let marked = find_path(start, end.clone(), move |node| {
        if node.x < 0 || node.y < 0 || is_wall(node.x, node.y) {
            return f64::INFINITY;
        }
        let s = distance(node, &end);
        s
    });

    //println!("Steps: {}", marked.as_ref().unwrap().len() - 1);

    //println!("{}", display_output_marked(3, 3, marked.unwrap()));
}
