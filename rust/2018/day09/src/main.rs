use marbles::MarblesGame;

mod marbles;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut game = MarblesGame::new(405, 71700);
    println!("High Score: {}", game.high_score());
}

fn part2() {
    let mut game = MarblesGame::new(405, 7170000);
    println!("High Score: {}", game.high_score());
}
