use std::collections::HashMap;

pub fn checksum_pair(id: impl AsRef<str>) -> (bool, bool) {
    let frequences = id.as_ref().chars().fold(HashMap::new(), |mut acc, a| {
        acc.entry(a).and_modify(|x| *x += 1).or_insert(1);
        acc
    });
    let has_two = frequences.values().any(|x| x == &2);
    let has_three = frequences.values().any(|x| x == &3);
    (has_two, has_three)
}

pub fn gen_checksum(ids: &Vec<String>) -> usize {
    let (twos, threes) =
        ids.iter()
            .map(|id| checksum_pair(id))
            .fold((0, 0), |mut acc, (has_two, has_three)| {
                if has_two {
                    acc.0 += 1;
                }
                if has_three {
                    acc.1 += 1;
                }
                (acc.0, acc.1)
            });
    twos * threes
}

pub fn id_diff(id_a: impl AsRef<str>, id_b: impl AsRef<str>) -> usize {
    id_a.as_ref()
        .chars()
        .zip(id_b.as_ref().chars())
        .filter(|(a, b)| a != b)
        .count()
}

pub fn find_id_pair(ids: &Vec<String>) -> Option<(&String, &String)> {
    for i in 0..ids.len() {
        for j in 0..ids.len() {
            if id_diff(&ids[i], &ids[j]) == 1 {
                return Some((&ids[i], &ids[j]));
            }
        }
    }
    None
}

pub fn find_common_in_pair(ids: &Vec<String>) -> Option<String> {
    let (a, b) = find_id_pair(ids)?;
    Some(
        a.chars()
            .zip(b.chars())
            .filter(|(c, d)| c == d)
            .map(|(a, _)| a)
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_count_zero() {
        let (has_two, has_three) = checksum_pair("abcdef");
        assert_eq!(has_two, false);
        assert_eq!(has_three, false);
    }

    #[test]
    fn can_count_two() {
        let (has_two, has_three) = checksum_pair("abcdef");
        assert_eq!(has_two, false);
        assert_eq!(has_three, false);
    }

    #[test]
    fn can_count_three() {
        let (has_two, has_three) = checksum_pair("abcdef");
        assert_eq!(has_two, false);
        assert_eq!(has_three, false);
    }

    #[test]
    fn can_count_both() {
        let (has_two, has_three) = checksum_pair("abcdef");
        assert_eq!(has_two, false);
        assert_eq!(has_three, false);
    }

    #[test]
    fn can_produce_checksum() {
        let ids = vec!["abcdef".to_owned(), "babebc".to_owned()];
        let checksum = gen_checksum(&ids);
        assert_eq!(checksum, 0);

        let ids = vec!["abcdef".to_owned(), "bababc".to_owned()];
        let checksum = gen_checksum(&ids);
        assert_eq!(checksum, 1);

        let ids = vec!["abadef".to_owned(), "bababc".to_owned()];
        let checksum = gen_checksum(&ids);
        assert_eq!(checksum, 2);

        let ids = vec!["ababaf".to_owned(), "bababc".to_owned()];
        let checksum = gen_checksum(&ids);
        assert_eq!(checksum, 4);
    }

    #[test]
    fn can_diff_ids() {
        assert!(id_diff("abcde", "axcye") == 2);
        assert!(id_diff("fghij", "fguij") == 1);
    }

    #[test]
    fn can_find_id_pair() {
        let ids = vec![
            "abcde".to_owned(),
            "fghij".to_owned(),
            "klmno".to_owned(),
            "pqrst".to_owned(),
            "fguij".to_owned(),
            "axcye".to_owned(),
            "wvxyz".to_owned(),
        ];

        let box_pair = find_id_pair(&ids);
        assert_eq!(box_pair, Some((&"fghij".to_owned(), &"fguij".to_owned())));

        let ids = vec![
            "abcde".to_owned(),
            "fghij".to_owned(),
            "klmno".to_owned(),
            "pqrst".to_owned(),
            "axcye".to_owned(),
            "wvxyz".to_owned(),
        ];

        let box_pair = find_id_pair(&ids);
        assert_eq!(box_pair, None);
    }

    #[test]
    fn can_find_common_in_pair() {
        let ids = vec![
            "abcde".to_owned(),
            "fghij".to_owned(),
            "klmno".to_owned(),
            "pqrst".to_owned(),
            "fguij".to_owned(),
            "axcye".to_owned(),
            "wvxyz".to_owned(),
        ];

        let box_pair = find_common_in_pair(&ids);
        assert_eq!(box_pair, Some("fgij".to_owned()));
    }
}
