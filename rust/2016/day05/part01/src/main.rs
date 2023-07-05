const DOOR_ID: &[u8; 8] = b"wtnhxymk";
//const DOOR_ID: &[u8; 3] = b"abc";

fn main() {
    let mut password = String::new();
    for i in 0..u32::MAX {
        let mut id = DOOR_ID.to_vec();
        id.append(&mut format!("{}", i).bytes().collect());
        let digest = md5::compute(id.clone());
        let computed_hash = format!("{:x}", digest);
        let prefix = computed_hash[..5].to_string();
        if prefix == "00000" {
            let pass_char = computed_hash.chars().collect::<Vec<_>>()[5];
            print!("{}", pass_char);
            password.push(pass_char);
        }

        if password.len() >= 8 {
            println!();
            break;
        }
    }
}
