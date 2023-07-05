const STARTING_STRING: &str = "00111101111101000";

fn gen_dragon_curve(length: usize) -> String {
    let mut a = STARTING_STRING.to_owned();
    while a.len() < length {
        let mut b = String::from_iter(a.clone().chars().rev());
        b = b.replace("1", "_");
        b = b.replace("0", "1");
        b = b.replace("_", "0");
        a.push('0');
        a.push_str(&b);
    }

    a[..length].to_owned()
}

fn gen_check_sum(s: impl AsRef<str>) -> String {
    let mut check_sum = s.as_ref().to_string();
    loop {
        let chars = check_sum.chars().collect::<Vec<_>>();
        check_sum.clear();
        let chunks = chars.chunks(2);
        for chunk in chunks {
            check_sum.push(if chunk[0] == chunk[1] { '1' } else { '0' });
        }
        if check_sum.len() % 2 != 0 {
            break;
        }
    }
    check_sum
}

fn main() {
    let output = gen_dragon_curve(272);
    let check_sum = gen_check_sum(&output);
    println!("Output: {}", output);
    println!("CS Output: {}", check_sum);
}
