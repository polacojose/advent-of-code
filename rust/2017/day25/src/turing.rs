use std::{collections::VecDeque, iter::repeat};

pub struct TuringMachine {
    pub tape: VecDeque<bool>,
    cursor_pos: usize,
}

impl Default for TuringMachine {
    fn default() -> Self {
        Self {
            tape: VecDeque::from_iter(repeat(false).take(10)),
            cursor_pos: (0),
        }
    }
}

impl TuringMachine {
    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos == 0 {
            self.tape.push_front(false);
        } else {
            self.cursor_pos -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_pos == self.tape.len() - 1 {
            self.tape.push_back(false)
        }
        self.cursor_pos += 1;
    }

    pub fn read(&self) -> bool {
        self.tape[self.cursor_pos]
    }

    pub fn write(&mut self, val: bool) {
        self.tape[self.cursor_pos] = val;
    }
}
