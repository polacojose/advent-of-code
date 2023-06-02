use std::fs;

#[derive(Debug)]
struct Dimensions {
    length: u32,
    width: u32,
    height: u32,
}

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    let mut total_required_paper = 0;
    for line in file_string.lines() {
        let mut file_string = line.split('x');

        let dimensions = Dimensions {
            length: file_string.next().unwrap().trim().parse::<u32>().unwrap(),
            width: file_string.next().unwrap().trim().parse::<u32>().unwrap(),
            height: file_string.next().unwrap().trim().parse::<u32>().unwrap(),
        };

        let paper_required_for_box = get_area(&dimensions) + get_smallest_side_area(&dimensions);
        total_required_paper += paper_required_for_box;

        println!("{:?}", dimensions);
        println!("Smallest Side {}", get_smallest_side_area(&dimensions));
        println!("Area: {}", get_area(&dimensions));
        println!("Paper required for box: {}", paper_required_for_box);
    }

    println!("Total required paper: {}", total_required_paper);
}

fn get_smallest_side_area(dimensions: &Dimensions) -> u32 {
    vec![
        (dimensions.length * dimensions.width),
        (dimensions.length * dimensions.height),
        (dimensions.width * dimensions.height),
    ]
    .iter()
    .min()
    .cloned()
    .unwrap()
}

fn get_area(dimensions: &Dimensions) -> u32 {
    2 * (dimensions.length * dimensions.width)
        + 2 * (dimensions.length * dimensions.height)
        + 2 * (dimensions.width * dimensions.height)
}
