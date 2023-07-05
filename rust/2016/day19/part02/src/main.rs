use std::collections::VecDeque;

#[inline]
fn find_josephus_recur(mut seats: Vec<usize>) -> usize {
    seats.remove(seats.len() / 2);

    if seats.len() == 1 {
        return *seats.first().unwrap();
    }

    let mut new_seats = seats.drain(1..).into_iter().collect::<Vec<usize>>();
    new_seats.append(&mut seats);
    return find_josephus_recur(new_seats);
}

fn find_josephus(num: usize) -> usize {
    let seats = (1..=num).collect::<Vec<usize>>();
    return find_josephus_recur(seats);
}

fn find_josephus_non_recur(num: usize) -> usize {
    let mut seats = (1..=num).collect::<VecDeque<usize>>();

    while seats.len() > 1 {
        println!("{}", seats.len());
        seats.remove(seats.len() / 2);

        seats.rotate_left(1);
    }
    return seats.pop_front().unwrap();
}

fn main() {
    println!("{}", find_josephus_non_recur(3014387));
}
