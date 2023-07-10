use part01::get_dense_hash;

fn main() {
    let mut total_squares = 0;
    for i in 0..128 {
        let out = get_dense_hash(format!("ugkiagan-{}", i), 256);
        let out = get_defrag_line(out);
        total_squares += out.chars().filter(|x| x == &'#').count();
    }
    println!("{}", total_squares);
}

fn get_defrag_line(dense_hash: String) -> String {
    dense_hash
        .chars()
        .into_iter()
        .map(|x| format!("{:04b}", i8::from_str_radix(&x.to_string(), 16).unwrap()))
        .collect::<String>()
        .chars()
        .into_iter()
        .map(|x| if x == '1' { '#' } else { '.' })
        .collect::<String>()
}
