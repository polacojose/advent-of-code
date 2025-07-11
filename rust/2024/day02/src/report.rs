use std::str::FromStr;

#[derive(Clone)]
pub enum Safety {
    Safe,
    Unsafe,
}

#[derive(Clone)]
pub struct RawReport {
    levels: Option<Vec<i32>>,
}

#[derive(Debug)]
pub struct UnableToParse;
impl FromStr for RawReport {
    type Err = UnableToParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .trim()
            .split_whitespace()
            .map(|l| l.trim().parse::<i32>().map_err(|_| UnableToParse).unwrap())
            .collect();
        Ok(RawReport {
            levels: Some(levels),
        })
    }
}

impl RawReport {
    pub fn process_report(&mut self, allow_error: bool) -> Report {
        let safety = levels_safety(
            &mut self
                .levels
                .as_ref()
                .expect("No levels")
                .iter()
                .copied()
                .collect(),
            allow_error,
        );
        Report { safety }
    }
}

pub struct Report {
    pub safety: Safety,
}

fn levels_safety(levels: &mut Vec<i32>, allow_error: bool) -> Safety {
    let safety = cycle_safety(levels);

    if matches!(safety, Safety::Safe) || !allow_error {
        return safety;
    }

    for i in 0..levels.len() {
        let mut v = levels.clone();
        v.remove(i);

        let safety = cycle_safety(&v);
        if matches!(safety, Safety::Safe) {
            return safety;
        }
    }

    Safety::Unsafe
}

fn cycle_safety(levels: &[i32]) -> Safety {
    enum Dir {
        Inc,
        Dec,
    }
    let dir = if levels[0] < levels[1] {
        Dir::Inc
    } else {
        Dir::Dec
    };

    let not_safe = levels.windows(2).any(|w| {
        let diff = match dir {
            Dir::Inc => w[1] - w[0],
            Dir::Dec => w[0] - w[1],
        };
        diff <= 0 || diff > 3
    });

    if not_safe {
        Safety::Unsafe
    } else {
        Safety::Safe
    }
}
