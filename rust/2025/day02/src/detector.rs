use crate::models::{IDRangeOldScanner, OutOfRange};

impl IDRangeOldScanner {
    fn contains(&self, n: u64) -> Result<bool, OutOfRange> {
        if n < self.start {
            return Err(OutOfRange::Before);
        } else if self.end < n {
            return Err(OutOfRange::After);
        }
        Ok(true)
    }

    fn digits<T: Into<u64>>(n: T) -> u64 {
        let mut n = n.into();
        let mut d = 0;
        while n > 0 {
            n /= 10;
            d += 1;
        }
        d
    }

    fn digit_factors<T: Into<u64>>(n: T) -> Vec<u64> {
        let n = n.into();
        if n <= 1 {
            return vec![1];
        }

        let mut factors = vec![1];

        let d = Self::digits(n);
        for f in 2..d {
            if d.rem_euclid(f) == 0 {
                factors.push(f);
            }
        }
        factors
    }

    fn first_half<N: Into<u64>>(n: N) -> u64 {
        let n = n.into();
        let d = Self::digits(n);
        Self::first_nth(n, d / 2)
    }

    fn first_nth<N: Into<u64>>(n: N, nth: u64) -> u64 {
        let n = n.into();
        let d = Self::digits(n);
        n / 10_u64.pow((d - nth) as u32)
    }

    fn expand_half<N: Into<u64>>(n: N) -> u64 {
        let n = n.into();
        let d = Self::digits(n);
        n * 10_u64.pow(d as u32) + n
    }

    fn expand_nth<N: Into<u64>>(r: N, n: N) -> u64 {
        let n = n.into();
        let r = r.into();
        let rd = Self::digits(r);
        let nd = Self::digits(n);

        assert!(nd.rem_euclid(rd) == 0);

        let mut s = 0;
        for i in (0..nd as usize).step_by(rd as usize).rev() {
            println!("i: {i}, r: {r}, s: {s} : {}", r * 10_u64.pow(i as u32));
            s = r * 10_u64.pow(i as u32) + s;
        }
        s
    }

    pub fn invalid_ids(&self) -> Vec<u64> {
        let mut invalid_ids = vec![];

        let mut cursor = self.start;

        println!("Testing Range: {:?}", self);

        loop {
            let num_digits = Self::digits(cursor);
            if num_digits % 2 != 0 {
                cursor = 10_u64.pow(num_digits as u32);
            }

            if cursor > self.end {
                break;
            }

            for kernel_width in Self::digit_factors(cursor) {
                let first_nth = Self::first_nth(cursor, kernel_width);
                let expand_nth = Self::expand_nth(first_nth, cursor);
                println!("cursor: {cursor:?}");
                println!("first_nth: {first_nth:?}");
                println!("expand_nth: {expand_nth:?}");
                match self.contains(expand_nth) {
                    Ok(_) => {
                        println!("{} in {:?}", expand_nth, self);
                        if !invalid_ids.contains(&expand_nth) {
                            invalid_ids.push(expand_nth);
                        }
                        cursor = cursor.max(expand_nth) + 1
                    }
                    Err(e) => match e {
                        OutOfRange::After => break,
                        _ => {}
                    },
                }
            }
        }

        invalid_ids
    }

    pub fn invalid_ids_half(&self) -> Vec<u64> {
        let mut invalid_ids = vec![];

        let mut cursor = self.start;
        let mut half_cursor = None;

        println!("Testing Range: {:?}", self);

        loop {
            let num_digits = Self::digits(cursor);
            if num_digits % 2 != 0 {
                cursor = 10_u64.pow(num_digits as u32);
            }

            if cursor > self.end {
                break;
            }

            if half_cursor.is_none() {
                half_cursor = Some(Self::first_half(cursor));
            }
            let half = half_cursor.unwrap();
            let expand_half = Self::expand_half(half);

            match self.contains(expand_half) {
                Ok(_) => {
                    println!("{} in {:?}", expand_half, self);
                    if !invalid_ids.contains(&expand_half) {
                        invalid_ids.push(expand_half);
                    }
                }
                Err(e) => match e {
                    OutOfRange::After => break,
                    _ => {}
                },
            }

            half_cursor = half_cursor.map(|a| a + 1);
            cursor = expand_half;
        }

        invalid_ids
    }
}

//#[cfg(test)]
//mod tests {
//    use crate::models::OutOfRange;
//
//    use super::*;
//
//    macro_rules! test_invalid_ids {
//        ($s: expr, $e: expr, $r: expr) => {
//            let d = IDRange { start: $s, end: $e };
//            assert_eq!(d.invalid_ids_half(), $r)
//        };
//    }
//
//    #[test]
//    fn test_first_nth() {
//        assert_eq!(IDRangeOldScanner::first_nth(12345_u64, 1), 1);
//        assert_eq!(IDRangeOldScanner::first_nth(12345_u64, 2), 12);
//        assert_eq!(IDRangeOldScanner::first_nth(12345_u64, 3), 123);
//        assert_eq!(IDRangeOldScanner::first_nth(12345_u64, 4), 1234);
//        assert_eq!(IDRangeOldScanner::first_nth(12345_u64, 5), 12345);
//    }
//
//    #[test]
//    fn test_expand_nth() {
//        assert_eq!(IDRangeOldScanner::expand_nth(12_u64, 123456), 121212);
//        assert_eq!(IDRangeOldScanner::expand_nth(12_u64, 1234), 1212);
//        assert_eq!(IDRangeOldScanner::expand_nth(123_u64, 123456789), 123123123);
//    }
//
//    #[test]
//    fn test_odd() {
//        let d = IDRangeOldScanner {
//            start: 123,
//            end: 124,
//        };
//        assert_eq!(d.invalid_ids_half(), vec![]);
//    }
//
//    #[test]
//    fn test_first_half() {
//        assert_eq!(IDRangeOldScanner::first_half(1234_u64), 12);
//        assert_eq!(IDRangeOldScanner::first_half(12345678_u64), 1234);
//        assert_eq!(IDRangeOldScanner::first_half(1188511880_u64), 11885);
//    }
//
//    #[test]
//    fn test_expand_half() {
//        assert_eq!(IDRangeOldScanner::expand_half(1234_u64), 12341234);
//        assert_eq!(IDRangeOldScanner::expand_half(12_u64), 1212);
//    }
//
//    #[test]
//    fn test_largest_digit_multiple() {
//        assert_eq!(IDRangeOldScanner::digit_factors(1234_u64), vec![1, 2]);
//        assert_eq!(IDRangeOldScanner::digit_factors(12345_u64), vec![1]);
//        assert_eq!(IDRangeOldScanner::digit_factors(123456_u64), vec![1, 2, 3]);
//    }
//
//    #[test]
//    fn test_range_contains() {
//        assert!(matches!(
//            IDRangeOldScanner { start: 37, end: 57 }.contains(33),
//            Err(OutOfRange::Before)
//        ));
//
//        assert!(matches!(
//            IDRangeOldScanner { start: 37, end: 57 }.contains(66),
//            Err(OutOfRange::After)
//        ));
//
//        assert!(matches!(
//            IDRangeOldScanner { start: 37, end: 57 }.contains(44),
//            Ok(true)
//        ));
//    }
//
//    #[test]
//    fn test_invalid_ids_full() {
//        let id_range = IDRangeOldScanner {
//            start: 11,
//            end: 300,
//        }
//        .invalid_ids();
//        println!("{id_range:?}")
//    }
//
//    #[test]
//    fn test_invalid_ids_half() {
//        test_invalid_ids!(11, 22, vec![11, 22]);
//        test_invalid_ids!(95, 115, vec![99]);
//        test_invalid_ids!(95, 115, vec![99]);
//        test_invalid_ids!(95, 115, vec![99]);
//        test_invalid_ids!(1188511880, 1188511890, vec![1188511885]);
//        test_invalid_ids!(222220, 222224, vec![222222]);
//        test_invalid_ids!(1698522, 1698528, vec![]);
//        test_invalid_ids!(446443, 446449, vec![446446]);
//        test_invalid_ids!(38593856, 38593862, vec![38593859]);
//    }
//
//    #[test]
//    fn test_invalid_ids_sum() {
//        let ranges: Result<Vec<IDRangeOldScanner>, String> = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".split(",").map(|s|s.parse()).collect();
//        let ranges = ranges.unwrap();
//
//        let sum = ranges.into_iter().fold(0, |sum, range| {
//            range.invalid_ids_half().into_iter().sum::<u64>() + sum
//        });
//
//        assert!(sum == 1227775554);
//    }
//
//    #[test]
//    fn failures() {
//        test_invalid_ids!(37, 57, vec![44, 55]);
//    }
//}
