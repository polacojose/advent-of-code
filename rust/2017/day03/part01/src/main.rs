use std::fs;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    for target_number in file_string.lines().map(|l| l.parse::<isize>().unwrap()) {
        let mut n_square: isize = 1;
        let mut steps: isize = 0;
        loop {
            let square_rb = n_square * n_square;
            if target_number <= square_rb {
                let rb_cord = (steps, steps);
                //println!("{n_square}->{square_rb} = ({steps},{steps})");
                //break;

                let mut new_cord = rb_cord.clone();

                let segments = [
                    n_square - 1,
                    (n_square - 1) * 2,
                    (n_square - 1) * 3,
                    (n_square - 1) * 4,
                ];

                if target_number >= square_rb - segments[0] {
                    new_cord.0 = new_cord.0 - (square_rb - target_number);
                } else if target_number >= square_rb - segments[1] {
                    new_cord.0 = new_cord.0 - segments[0];
                    new_cord.1 = new_cord.1 - ((square_rb - segments[0]) - target_number);
                } else if target_number >= square_rb - segments[2] {
                    new_cord.1 = new_cord.1 - segments[0];
                    new_cord.0 =
                        (new_cord.0 - segments[0]) + ((square_rb - segments[1]) - target_number);
                } else if target_number >= square_rb - segments[3] {
                    new_cord.1 =
                        (new_cord.1 - segments[0]) + ((square_rb - segments[2]) - target_number)
                }

                println!("{target_number} : {:?} ", new_cord);
                println!("Distance = {}", new_cord.0.abs() + new_cord.1.abs());

                break;
            }

            steps += 1;
            n_square += 2;
        }
    }
}
