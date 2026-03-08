use std::collections::HashSet;

#[inline]
pub fn max_of_digit(n: impl Into<u32> + Copy) -> u64 {
    10_u64.pow(n.into()) - 1_u64
}

#[inline]
pub fn min_of_digit(n: impl Into<u32> + Copy) -> u64 {
    10_u64.pow(n.into() - 1)
}

pub fn digits<T: Into<u64>>(n: T) -> u8 {
    let mut n = n.into();
    let mut d = 0;
    while n > 0 {
        n /= 10;
        d += 1;
    }
    d
}

pub fn kernels_by_digit(d: impl Into<u8>) -> HashSet<u8> {
    let d = d.into();

    let mut kernels = [1].into_iter().collect::<HashSet<u8>>();

    for i in 2..d {
        if d.rem_euclid(i) == 0 {
            kernels.insert(i);
        }
    }

    if kernels.len() > 1 {
        kernels.remove(&1);
    }

    kernels
}

pub fn expand_nth<N: Into<u64>>(n: N, d: N) -> Result<u64, ()> {
    let d = d.into();
    let n = n.into();
    let nd = digits(n);

    if d.rem_euclid(nd.into()) != 0 {
        return Err(());
    }

    let mut s = 0;
    for i in (0..d as u32).step_by(nd as usize).rev() {
        s = n * 10_u64.pow(i as u32) + s;
    }

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_expand_nth {
        ($n: expr, $d: expr, $r: expr) => {
            assert_eq!(expand_nth($n as u64, $d as u64), $r);
        };
    }

    macro_rules! assert_can_get_kernel_by_digit {
        ($d: expr, $r: expr) => {
            assert!(kernels_by_digit($d) == $r.into_iter().collect::<HashSet<u8>>());
        };
    }

    #[test]
    pub fn test_can_expand_nth() {
        assert_expand_nth!(1, 5, Ok(11111));
        assert_expand_nth!(2, 5, Ok(22222));
        assert_expand_nth!(23, 6, Ok(232323));
        assert_expand_nth!(824, 9, Ok(824824824));
    }

    #[test]
    pub fn test_will_not_expand_invalid() {
        assert_expand_nth!(21, 5, Err(()));
        assert_expand_nth!(33, 5, Err(()));
        assert_expand_nth!(666666, 10, Err(()));
    }

    #[test]
    fn test_can_get_kernel_by_digit() {
        assert_can_get_kernel_by_digit!(2, [1]);
        assert_can_get_kernel_by_digit!(3, [1]);
        assert_can_get_kernel_by_digit!(4, [2]);
        assert_can_get_kernel_by_digit!(5, [1]);
        assert_can_get_kernel_by_digit!(6, [2, 3]);
        assert_can_get_kernel_by_digit!(7, [1]);
        assert_can_get_kernel_by_digit!(8, [2, 4]);
        assert_can_get_kernel_by_digit!(9, [3]);
        assert_can_get_kernel_by_digit!(10, [2, 5]);
    }
}
