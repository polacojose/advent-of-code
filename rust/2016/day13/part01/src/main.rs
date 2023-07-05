use float_ord::FloatOrd;
use part01::{
    astar::{distance, find_path, AStarVector},
    office::{display_output, display_output_marked, is_wall},
};

const OFFICE_DESIGNERS_FAVORITE_NUMBER: usize = 1350;

fn main() {
    println!(
        "{}",
        display_output(31, 39, OFFICE_DESIGNERS_FAVORITE_NUMBER)
    );

    let start = AStarVector {
        f_score: Some(FloatOrd { 0: 0.0 }),
        x: 1,
        y: 1,
    };
    let end = AStarVector {
        f_score: Some(FloatOrd { 0: 0.0 }),
        x: 31,
        y: 39,
    };

    let marked = find_path(start, end.clone(), move |node| {
        if node.x < 0
            || node.y < 0
            || is_wall(
                node.x as usize,
                node.y as usize,
                OFFICE_DESIGNERS_FAVORITE_NUMBER,
            )
        {
            return f64::INFINITY;
        }
        let s = distance(node, &end);
        s
    });

    println!("Steps: {}", marked.as_ref().unwrap().len() - 1);

    println!(
        "{}",
        display_output_marked(40, 40, OFFICE_DESIGNERS_FAVORITE_NUMBER, marked.unwrap())
    );
}
