use part02::{password_rules, AlphaNum};

fn main() {
    let mut alpha_num: AlphaNum = "hxbxwxba".parse().unwrap();
    println!("{:?}", alpha_num);
    while !password_rules::valid_password(alpha_num.get_alpha()) {
        alpha_num.increment();
    }
    println!("{:?}", alpha_num);
    alpha_num.increment();
    while !password_rules::valid_password(alpha_num.get_alpha()) {
        alpha_num.increment();
    }
    println!("{:?}", alpha_num);
}
