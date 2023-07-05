use part01::{
    self,
    password_rules::{self, valid_password},
    AlphaNum,
};

fn main() {
    let mut alpha_num: AlphaNum = "hxbxwxba".parse().unwrap();
    while !password_rules::valid_password(alpha_num.get_alpha()) {
        alpha_num.increment();
    }
    println!("{:?}", alpha_num);
}
