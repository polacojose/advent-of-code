use std::fs;

fn main() {
    let triangles = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| {
            x.split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    let correct_triangles = triangles
        .into_iter()
        .map(|triangle_parts| {
            let (a_one, a_two, a_three) = (
                triangle_parts[0][0],
                triangle_parts[0][1],
                triangle_parts[0][2],
            );
            let (b_one, b_two, b_three) = (
                triangle_parts[1][0],
                triangle_parts[1][1],
                triangle_parts[1][2],
            );
            let (c_one, c_two, c_three) = (
                triangle_parts[2][0],
                triangle_parts[2][1],
                triangle_parts[2][2],
            );
            vec![
                (a_one, b_one, c_one),
                (a_two, b_two, c_two),
                (a_three, b_three, c_three),
            ]
        })
        .flatten()
        .map(|triangle| {
            println!("Triangle = {:?}", triangle);
            let (a, b, c) = (triangle.0, triangle.1, triangle.2);
            a + b > c && b + c > a && a + c > b
        })
        .filter(|r| *r)
        .count();

    println!("{}", correct_triangles);
}
