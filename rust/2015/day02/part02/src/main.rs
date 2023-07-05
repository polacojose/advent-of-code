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
    let mut total_required_ribbon = 0;
    for line in file_string.lines() {
        let mut file_string = line.split('x');

        let dimensions = Dimensions {
            length: file_string.next().unwrap().trim().parse::<u32>().unwrap(),
            width: file_string.next().unwrap().trim().parse::<u32>().unwrap(),
            height: file_string.next().unwrap().trim().parse::<u32>().unwrap(),
        };

        let paper_required_for_box = get_area(&dimensions) + get_smallest_side_area(&dimensions);
        total_required_paper += paper_required_for_box;

        let ribbon_required_for_box = get_total_ribbon_required(&dimensions);
        total_required_ribbon += ribbon_required_for_box;

        println!("{:?}", dimensions);
        println!("Smallest Side {}", get_smallest_side_area(&dimensions));
        println!("Area: {}", get_area(&dimensions));
        println!("Paper required for box: {}", paper_required_for_box);
        println!("Ribbon required for box: {}", ribbon_required_for_box);
    }

    println!("Total required paper: {}", total_required_paper);
    println!("Total required ribbon: {}", total_required_ribbon);
}

fn get_total_ribbon_required(dimensions: &Dimensions) -> u32 {
    get_smallest_side_parameter(dimensions) + get_volume(dimensions)
}

fn get_volume(dimensions: &Dimensions) -> u32 {
    dimensions.length * dimensions.width * dimensions.height
}

fn get_smallest_side_parameter(dimensions: &Dimensions) -> u32 {
    vec![
        (2 * dimensions.length + 2 * dimensions.width),
        (2 * dimensions.length + 2 * dimensions.height),
        (2 * dimensions.width + 2 * dimensions.height),
    ]
    .iter()
    .min()
    .cloned()
    .unwrap()
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
