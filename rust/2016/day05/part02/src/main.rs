use std::collections::HashSet;

const DOOR_ID: &[u8; 8] = b"wtnhxymk";
//const DOOR_ID: &[u8; 3] = b"abc";

fn main() {
    let mut touched_positions = HashSet::new();
    let mut password: [char; 8] = ['0'; 8];
    for i in 0..u32::MAX {
        let mut id = DOOR_ID.to_vec();
        id.append(&mut format!("{}", i).bytes().collect());
        let digest = md5::compute(id.clone());
        let computed_hash = format!("{:x}", digest);
        let prefix = computed_hash[..5].to_string();
        if prefix == "00000" {
            let pass_pos = computed_hash.chars().collect::<Vec<_>>()[5]
                .to_string()
                .parse::<usize>();

            if let Ok(pass_pos) = pass_pos {
                if pass_pos >= 8 {
                    continue;
                }
                if touched_positions.contains(&pass_pos) {
                    continue;
                }

                touched_positions.insert(pass_pos);
                let pass_char = computed_hash.chars().collect::<Vec<_>>()[6];
                password[pass_pos] = pass_char;
                println!("{}", String::from_iter(password));
            }
        }

        if touched_positions.len() >= 8 {
            println!();
            break;
        }
    }
}
