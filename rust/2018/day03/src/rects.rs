use std::{collections::HashMap, str::FromStr};

use regex::Regex;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Vector2D {
    x: i64,
    y: i64,
}

#[derive(PartialEq, Eq, Clone)]
struct Rect {
    pos: Vector2D,
    width: i64,
    height: i64,
}

impl From<&Claim> for Rect {
    fn from(value: &Claim) -> Self {
        value.rect.clone()
    }
}

#[derive(PartialEq, Eq)]
pub struct Claim {
    id: i64,
    rect: Rect,
}

#[derive(Debug)]
pub struct UnableToParseClaim;
impl FromStr for Claim {
    type Err = UnableToParseClaim;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").map_err(|_| UnableToParseClaim)?;
        let Some(caps) = re.captures(s) else {
            return Err(UnableToParseClaim);
        };

        Ok(Claim {
            id: caps[1].parse().unwrap(),
            rect: Rect {
                pos: Vector2D {
                    x: caps[2].parse().unwrap(),
                    y: caps[3].parse().unwrap(),
                },
                width: caps[4].parse().unwrap(),
                height: caps[5].parse().unwrap(),
            },
        })
    }
}

fn rect_areas(rect: &Rect) -> Vec<Vector2D> {
    let mut areas = Vec::new();
    for y in rect.pos.y..rect.pos.y + rect.height {
        for x in rect.pos.x..rect.pos.x + rect.width {
            areas.push(Vector2D { x, y })
        }
    }
    areas
}

fn overlapping_areas(claims: &[Claim]) -> Vec<Vector2D> {
    claims
        .into_iter()
        .fold(HashMap::new(), |mut acc, val| {
            for a in rect_areas(&val.rect) {
                acc.entry(a).and_modify(|x| *x += 1).or_insert(1);
            }
            acc
        })
        .into_iter()
        .filter(|(_, c)| c > &1)
        .map(|(v, _)| v)
        .collect()
}

pub fn overlapping_areas_count(claims: &[Claim]) -> usize {
    overlapping_areas(claims).len()
}

pub fn non_overlapping_claim_id(claims: &[Claim]) -> Option<i64> {
    let oa = overlapping_areas(claims);
    'claims: for c in claims {
        let c_areas = rect_areas(&c.rect);
        for a in c_areas {
            if oa.contains(&a) {
                continue 'claims;
            }
        }
        return Some(c.id);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_claim() {
        let expected = Claim {
            id: 1,
            rect: Rect {
                pos: Vector2D { x: 1, y: 3 },
                width: 4,
                height: 4,
            },
        };
        assert!("#1 @ 1,3: 4x4".parse::<Claim>().unwrap() == expected);

        let expected = Claim {
            id: 2,
            rect: Rect {
                pos: Vector2D { x: 3, y: 1 },
                width: 4,
                height: 4,
            },
        };
        assert!("#2 @ 3,1: 4x4".parse::<Claim>().unwrap() == expected);

        let expected = Claim {
            id: 3,
            rect: Rect {
                pos: Vector2D { x: 5, y: 5 },
                width: 2,
                height: 2,
            },
        };
        assert!("#3 @ 5,5: 2x2".parse::<Claim>().unwrap() == expected);

        let expected = Claim {
            id: 4,
            rect: Rect {
                pos: Vector2D { x: 5, y: 5 },
                width: 2,
                height: 3,
            },
        };
        assert!("#4 @ 5,5: 2x3".parse::<Claim>().unwrap() == expected);
    }

    #[test]
    fn can_get_rect_areas() {
        let expected = vec![
            Vector2D { x: 0, y: 0 },
            Vector2D { x: 1, y: 0 },
            Vector2D { x: 0, y: 1 },
            Vector2D { x: 1, y: 1 },
        ];
        let claim = "#3 @ 0,0: 2x2".parse::<Claim>().unwrap();
        assert!(rect_areas(&claim.rect) == expected);

        let expected = vec![
            Vector2D { x: 1, y: 1 },
            Vector2D { x: 2, y: 1 },
            Vector2D { x: 1, y: 2 },
            Vector2D { x: 2, y: 2 },
        ];
        let claim = "#3 @ 1,1: 2x2".parse::<Claim>().unwrap();
        assert!(rect_areas(&claim.rect) == expected);

        let expected = vec![
            Vector2D { x: 1, y: 1 },
            Vector2D { x: 2, y: 1 },
            Vector2D { x: 3, y: 1 },
            Vector2D { x: 1, y: 2 },
            Vector2D { x: 2, y: 2 },
            Vector2D { x: 3, y: 2 },
            Vector2D { x: 1, y: 3 },
            Vector2D { x: 2, y: 3 },
            Vector2D { x: 3, y: 3 },
        ];
        let claim = "#3 @ 1,1: 3x3".parse::<Claim>().unwrap();
        assert!(rect_areas(&claim.rect) == expected);
    }

    #[test]
    fn can_get_overlap_areas_count() {
        let claim_a = "#1 @ 1,3: 4x4".parse::<Claim>().unwrap();
        let claim_b = "#2 @ 3,1: 4x4".parse::<Claim>().unwrap();
        let claim_c = "#3 @ 5,5: 2x2".parse::<Claim>().unwrap();
        assert!(overlapping_areas_count(&vec![claim_a, claim_b, claim_c]) == 4);

        let claim_a = "#1 @ 1,3: 4x4".parse::<Claim>().unwrap();
        let claim_b = "#2 @ 3,1: 4x4".parse::<Claim>().unwrap();
        let claim_c = "#3 @ 4,5: 2x2".parse::<Claim>().unwrap();
        assert!(overlapping_areas_count(&vec![claim_a, claim_b, claim_c]) == 6);
    }

    #[test]
    fn find_non_overlapping_claim_id() {
        let claim_a = "#1 @ 1,3: 4x4".parse::<Claim>().unwrap();
        let claim_b = "#2 @ 3,1: 4x4".parse::<Claim>().unwrap();
        let claim_c = "#3 @ 5,5: 2x2".parse::<Claim>().unwrap();
        assert!(non_overlapping_claim_id(&vec![claim_a, claim_b, claim_c]) == Some(3));

        let claim_a = "#1 @ 1,3: 4x4".parse::<Claim>().unwrap();
        let claim_b = "#2 @ 3,1: 4x4".parse::<Claim>().unwrap();
        let claim_c = "#3 @ 4,5: 2x2".parse::<Claim>().unwrap();
        assert!(non_overlapping_claim_id(&vec![claim_a, claim_b, claim_c]) == None);
    }
}
