fn main() {
    //part1();
    part2();
}

fn part1() {
    let step_size: usize = 303;
    let mut spin_lock: Vec<usize> = vec![0];
    let mut location: usize = 0;

    for i in 1..=2017 {
        location = ((location + step_size) % spin_lock.len()) + 1;
        spin_lock.insert(location, i);
    }
    println!("Part01 after {:?}", spin_lock.get(location + 1));
}

fn part2() {
    let step_size: usize = 303;
    let mut spin_lock_len = 1;
    let mut location: usize = 0;

    for i in 1..=50000000 {
        location = ((location + step_size) % spin_lock_len) + 1;
        spin_lock_len += 1;
        if location == 1 {
            println!("{}", i);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        const step_size: usize = 3;
        let mut spin_lock: Vec<usize> = vec![0];
        let mut location: usize = 0;

        for i in 1..=2017 {
            location = ((location + step_size) % spin_lock.len()) + 1;
            spin_lock.insert(location, i);
        }
        println!("After {:?}", spin_lock.get(location + 1));
    }
}
