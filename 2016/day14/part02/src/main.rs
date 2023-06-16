use std::{collections::HashMap, sync::RwLock};

use lazy_static::lazy_static;

const SALT: &str = "ihaygndm";

lazy_static! {
    static ref DIGEST_MAP: RwLock<HashMap<usize, String>> = Default::default();
}

fn find_index_of_nth_key(nth: usize) -> Result<usize, ()> {
    let mut keys_found = 0;
    for i in 0.. {
        let digest_string = produce_deep_hash(i);

        if let Ok(c) = has_triple(digest_string.clone()) {
            if is_key(i + 1, c) {
                keys_found += 1;
                println!("{keys_found} = {i}");
                if keys_found >= nth {
                    return Ok(i);
                }
            }
        }
    }
    Err(())
}

fn produce_deep_hash(i: usize) -> String {
    let entry = DIGEST_MAP.read().unwrap().get(&i).cloned();
    if let Some(digest_string) = entry {
        return digest_string.clone();
    } else {
        let mut digest_string = format!("{}{}", SALT, i);
        for _ in 0..2017 {
            let digest = md5::compute(digest_string);
            digest_string = format!("{:?}", digest);
        }
        DIGEST_MAP.write().unwrap().insert(i, digest_string.clone());
        return digest_string;
    }
}

fn is_key(trial_key_index: usize, c: char) -> bool {
    for i in trial_key_index..(trial_key_index + 1000) {
        let digest_string = produce_deep_hash(i);
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
