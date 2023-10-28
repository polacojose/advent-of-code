use crate::turing::TuringMachine;

#[derive(Debug, Clone)]
pub(super) enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub(super) struct StateCondition {
    pub(super) cond_val: bool,
    pub(super) write_val: bool,
    pub(super) move_dir: Dir,
    pub(super) next_state_id: String,
}

#[derive(Debug, Clone)]
pub(super) struct State {
    pub(super) id: String,
    pub(super) condition_a: StateCondition,
    pub(super) condition_b: StateCondition,
}

#[derive(Debug)]
pub struct TuringDiagnostic {
    pub(super) current_state_id: String,
    pub(super) steps: u64,
    pub(super) states: Vec<State>,
}

impl TuringDiagnostic {
    pub fn run_diagnostic(&mut self, machine: &mut TuringMachine) {
        for _ in 0..self.steps {
            let cur_state = self.state(&self.current_state_id).unwrap();
            let condition = if machine.read() == cur_state.condition_a.cond_val {
                cur_state.condition_a
            } else {
                cur_state.condition_b
            };

            machine.write(condition.write_val);
            match condition.move_dir {
                Dir::Left => machine.move_cursor_left(),
                Dir::Right => machine.move_cursor_right(),
            }
            self.current_state_id = condition.next_state_id;
        }
        println!(
            "Checksum is {}.",
            machine.tape.iter().filter(|b| **b).count()
        );
    }

    fn state(&self, id: &String) -> Option<State> {
        for state in &self.states {
            if &state.id == id {
                return Some(state.clone());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn turing_diag_parsing() {
        let turing_diagnostic: TuringDiagnostic =
            fs::read_to_string("input.txt").unwrap().parse().unwrap();

        println!("{:#?}", turing_diagnostic);
    }
}
