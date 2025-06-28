#[derive(Debug, PartialEq, Eq)]
pub enum Letter {
    X,
    M,
    A,
    S,
}

impl Letter {
    pub fn next(&self) -> Option<Self> {
        match self {
            Letter::X => Some(Letter::M),
            Letter::M => Some(Letter::A),
            Letter::A => Some(Letter::S),
            Letter::S => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_letter {
        ($first:expr, $second:expr) => {
            assert_eq!($first.next(), $second);
        };
    }

    #[test]
    fn test_next_letter() {
        test_letter!(Letter::X, Some(Letter::M));
        test_letter!(Letter::A, Some(Letter::S));
        test_letter!(Letter::S, None);
    }
}
