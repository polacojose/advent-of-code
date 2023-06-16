use float_ord::FloatOrd;
use part02::office::OFFICE_DESIGNERS_FAVORITE_NUMBER;
use part02::{
    astar::{distance, find_path, AStarVector},
    office::is_wall,
};

fn main() {
    let mut under_eq_50 = 0;

    let start = AStarVector {
        f_score: Some(FloatOrd { 0: 0.0 }),
        x: 1,
        y: 1,
    };

    for y in 0..50 {
        for x in 0..50 {
            // println!(
            //     "{}",
            //     display_output(31, 39, OFFICE_DESIGNERS_FAVORITE_NUMBER)
            // );

            let end = AStarVector {
                f_score: Some(FloatOrd { 0: 0.0 }),
                x,
                y,
            };

            let marked = find_path(start.clone(), end.clone(), move |node| {
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

            match marked {
                Ok(_) => {
                    under_eq_50 += 1;
                    println!("<= 50 = {}", under_eq_50);
                }
                _ => {}
            }
        }
    }
}
