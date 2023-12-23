struct Race {
    time: usize,
    distance: usize,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let r1 = Race {
        time: 51,
        distance: 222,
    };
    let r2 = Race {
        time: 92,
        distance: 2031,
    };
    let r3 = Race {
        time: 68,
        distance: 1126,
    };
    let r4 = Race {
        time: 90,
        distance: 1225,
    };

    println!(
        "Part1 Total = {}",
        num_solutions_race(r1)
            * num_solutions_race(r2)
            * num_solutions_race(r3)
            * num_solutions_race(r4)
    );
}

fn part2() {
    let r1 = Race {
        time: 51_92_68_90,
        distance: 222_2031_1126_1225,
    };

    println!("Part2 Total = {}", num_solutions_race(r1));
}

fn num_solutions_race(race: Race) -> usize {
    let (plus, minus) = quadratic(-1.0, race.time as f64, -(race.distance as f64));
    let min = plus.min(minus);
    let max = plus.max(minus);

    let mut solutions = max.floor() - min.ceil();
    if max % 1.0 == 0.0 {
        solutions = solutions - 1.0;
    }
    if min % 1.0 != 0.0 {
        solutions = solutions + 1.0;
    }
    solutions as usize
}

fn quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let plus = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    let minus = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    (plus, minus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_solutions() {
        let r1 = num_solutions_race(Race {
            time: 7,
            distance: 9,
        });
        let r2 = num_solutions_race(Race {
            time: 15,
            distance: 40,
        });
        let r3 = num_solutions_race(Race {
            time: 30,
            distance: 200,
        });

        assert_eq!(r1, 4);
        assert_eq!(r2, 8);
        assert_eq!(r3, 9);

        assert_eq!(r1 * r2 * r3, 288)
    }

    #[test]
    fn can_find_solutions_p2() {
        let r1 = num_solutions_race(Race {
            time: 7_15_30,
            distance: 9_40_200,
        });
        assert_eq!(r1, 71503);
    }
}
