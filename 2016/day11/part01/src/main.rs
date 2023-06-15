use std::collections::HashSet;

use part01::game::{GameState, INITIAL_GAME_STATE};

fn main() {
    let game_state = (*INITIAL_GAME_STATE).clone();
    let mut state_queue: HashSet<GameState> = Default::default();
    state_queue.insert(game_state);
    let mut depth = 0;
    loop {
        println!("Depth: {}", depth);
        println!("Search Space: {}", state_queue.len());
        let mut found = false;
        for state in state_queue.iter() {
            if state.is_victory_state() {
                found = true;
                break;
            }
        }

        if found {
            break;
        }

        let mut new_state_queue: HashSet<GameState> = Default::default();
        for state in state_queue.drain() {
            let next_possible_states = state.get_next_valid_states();
            for next_state in next_possible_states {
                new_state_queue.insert(next_state);
            }
        }
        state_queue = new_state_queue;
        depth += 1;
    }
}
