use std::collections::HashSet;

use crate::{
    digit::{digits, expand_nth, kernels_by_digit, max_of_digit, min_of_digit},
    models::IDRange,
};

#[cfg(test)]
fn invalid_ids_at_nth_digit(n: impl Into<u8>) -> Vec<u64> {
    let n = n.into();
    let kernel = kernels_by_digit(n);

    let mut invalid_ids = vec![];

    for k in kernel {
        for i in min_of_digit(k)..=max_of_digit(k) {
            if let Ok(r) = expand_nth(i, n.into()) {
                invalid_ids.push(r);
            } else {
                break;
            }
        }
    }

    invalid_ids
}

pub fn invalid_ids_at_in_id_range(idrange: &IDRange) -> Vec<u64> {
    let mut kernel = kernels_by_digit(digits(idrange.start));
    kernel.extend(kernels_by_digit(digits(idrange.end)));

    let sd = digits(idrange.start);
    let ed = digits(idrange.end);

    let mut invalid_ids = HashSet::new();
    for k in kernel {
        for i in min_of_digit(k)..=max_of_digit(k) {
            if sd > 1 {
                if let Ok(r) = expand_nth(i, sd.into()) {
                    if idrange.contains(r).is_ok() {
                        invalid_ids.insert(r);
                    }
                }
            }
            if let Ok(r) = expand_nth(i, ed.into()) {
                if idrange.contains(r).is_ok() {
                    invalid_ids.insert(r);
                }
            }
        }
    }

    let mut arr = invalid_ids.into_iter().collect::<Vec<_>>();
    arr.sort();
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_invalid_ids {
        ($s: expr, $e: expr, $r: expr) => {{
            let d = IDRange { start: $s, end: $e };
            let result = invalid_ids_at_in_id_range(&d);
            assert_eq!(result, $r)
        }};
    }

    macro_rules! assert_invalid_ids_at_nth_digit {
        ($e: expr, $r: expr) => {{
            let result = invalid_ids_at_nth_digit($e);
            assert!($r.into_iter().all(|n| result.contains(&n)));
        }};
    }

    #[test]
    fn test_can_get_invalid_ids_at_nth_digit() {
        assert_invalid_ids_at_nth_digit!(2, [11, 55, 99]);
        assert_invalid_ids_at_nth_digit!(3, [111, 555, 999]);
        assert_invalid_ids_at_nth_digit!(4, [1111, 1010, 3333, 4949]);
        assert_invalid_ids_at_nth_digit!(9, [824824824]);
        assert_invalid_ids_at_nth_digit!(10, [2121212121]);
    }

    #[test]
    fn test_can_get_invalid_ids_from_range() {
        test_invalid_ids!(1, 17, [11]);
        test_invalid_ids!(11, 22, [11, 22]);
        test_invalid_ids!(95, 115, [99, 111]);
        test_invalid_ids!(998, 1012, [999, 1010]);
        test_invalid_ids!(1188511880, 1188511890, [1188511885]);
        test_invalid_ids!(222220, 222224, [222222]);
        test_invalid_ids!(1698522, 1698528, []);
        test_invalid_ids!(446443, 446449, [446446]);
        test_invalid_ids!(38593856, 38593862, [38593859]);
        test_invalid_ids!(565653, 565659, [565656]);
        test_invalid_ids!(824824821, 824824827, [824824824]);
        test_invalid_ids!(2121212118, 2121212124, [2121212121]);
    }

    #[test]
    fn test_can_sum() {
        let ranges: Vec<IDRange> =
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();

        let sum = ranges
            .into_iter()
            .map(|r| invalid_ids_at_in_id_range(&r).into_iter().sum::<u64>())
            .sum::<u64>();
        assert!(sum == 4174379265);
    }
}
