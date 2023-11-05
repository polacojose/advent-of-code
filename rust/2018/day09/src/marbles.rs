use std::collections::VecDeque;

struct Player {
    score: usize,
}

pub struct MarblesGame {
    last_marble: usize,
    player_index: usize,
    current_marble_number: u32,
    players: Vec<Player>,
    marbles: VecDeque<u32>,
}

impl MarblesGame {
    pub fn new(players: usize, last_marble: usize) -> Self {
        let players = (0..players).map(|_| Player { score: 0 }).collect();
        let mut marbles = VecDeque::with_capacity(last_marble);
        marbles.append(&mut vec![0, 1].into());
        Self {
            players,
            last_marble,
            player_index: 1,
            current_marble_number: 2,
            marbles,
        }
    }

    fn resolve(&mut self) {
        while (self.current_marble_number as usize) <= self.last_marble + 1 {
            self.player_index %= self.players.len();

            if self.current_marble_number % 23 == 0 {
                self.players[self.player_index].score += self.current_marble_number as usize;

                self.marbles.rotate_right(7);
                let val = self.marbles.pop_back().unwrap();
                self.marbles.rotate_left(1);

                self.players[self.player_index].score += val as usize;
            } else {
                self.marbles.rotate_left(1);
                self.marbles.push_back(self.current_marble_number);
            }

            self.current_marble_number += 1;
            self.player_index += 1;
        }
    }

    pub fn high_score(&mut self) -> usize {
        self.resolve();
        self.players.iter().max_by_key(|x| x.score).unwrap().score
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! assert_high_score {
        ($players:expr, $last_marble:expr, $high_score:expr) => {
            let mut game = MarblesGame::new($players, $last_marble);
            assert_eq!(game.high_score(), $high_score);
        };
    }

    #[test]
    fn can_get_high_score() {
        assert_high_score!(9, 25, 32);
        assert_high_score!(10, 1618, 8317);
        assert_high_score!(13, 7999, 146373);
        assert_high_score!(17, 1104, 2764);
        assert_high_score!(21, 6111, 54718);
        assert_high_score!(30, 5807, 37305);
    }
}
