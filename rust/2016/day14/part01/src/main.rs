const SALT: &str = "ihaygndm";

fn find_index_of_nth_key(nth: usize) -> Result<usize, ()> {
    let mut keys_found = 0;
    for i in 0.. {
        let digest = md5::compute(format!("{}{}", SALT, i));
        let digest_string = format!("{:?}", digest);
        if let Ok(c) = has_triple(digest_string.clone()) {
            if is_key(i + 1, c) {
                keys_found += 1;
                if keys_found >= nth {
                    return Ok(i);
                }
            }
        }
    }
    Err(())
}

fn is_key(trial_key_index: usize, c: char) -> bool {
    for i in trial_key_index..(trial_key_index + 1000) {
        let digest = md5::compute(format!("{}{}", SALT, i));
        let digest_string = format!("{:?}", digest);
        if has_five(digest_string, c).is_ok() {
            return true;
        }
    }

    false
}

fn has_triple(s: impl AsRef<str>) -> Result<char, ()> {
    let chars = s.as_ref().chars();
    let ref_vec = chars.collect::<Vec<_>>();
    for i in 2..ref_vec.len() {
        if ref_vec[i - 2] == ref_vec[i - 1] && ref_vec[i - 1] == ref_vec[i] {
            return Ok(ref_vec[i]);
        }
    }
    Err(())
}

fn has_five(s: impl AsRef<str>, c: char) -> Result<char, ()> {
    let chars = s.as_ref().chars();
    let ref_vec = chars.collect::<Vec<_>>();
    for i in 4..ref_vec.len() {
        if ref_vec[i] == c
            && ref_vec[i - 4] == ref_vec[i - 3]
            && ref_vec[i - 3] == ref_vec[i - 2]
            && ref_vec[i - 2] == ref_vec[i - 1]
            && ref_vec[i - 1] == ref_vec[i]
        {
            return Ok(ref_vec[i]);
        }
    }
    Err(())
}

fn main() {
    println!("{:?}", find_index_of_nth_key(64));
}
