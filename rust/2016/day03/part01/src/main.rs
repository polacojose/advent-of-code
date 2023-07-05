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
        .collect::<Vec<_>>();

    let correct_triangles = triangles
        .into_iter()
        .map(|triangle| {
            let (a, b, c) = (triangle[0], triangle[1], triangle[2]);
            a + b > c && b + c > a && a + c > b
        })
        .filter(|r| *r)
        .count();

    println!("{}", correct_triangles);
}
