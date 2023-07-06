pub struct GroupScoreStringCounter<'a> {
    reference_string: &'a str,
    pub score: usize,
    pub garbage_count: usize,
    pos: usize,
    depth: usize,
    state: GroupScoreStringCounterState,
}

#[derive(Debug)]
enum GroupScoreStringCounterState {
    Init,
    GroupOpen,
    GroupClose,
    GarbageSkip,
    Done,
}

impl<'a> GroupScoreStringCounter<'a> {
    pub fn new(reference_string: &'a str) -> Self {
        Self {
            reference_string,
            score: 0,
            garbage_count: 0,
            pos: 0,
            depth: 0,
            state: GroupScoreStringCounterState::Init,
        }
    }

    pub fn scan(&mut self) -> bool {
        self.state = match self.state {
            GroupScoreStringCounterState::Init => self.init_scan(),
            GroupScoreStringCounterState::GroupOpen => self.group_open(),
            GroupScoreStringCounterState::GroupClose => self.group_close(),
            GroupScoreStringCounterState::GarbageSkip => self.garbage_skip(),
            GroupScoreStringCounterState::Done => GroupScoreStringCounterState::Done,
        };

        match self.state {
            GroupScoreStringCounterState::Done => false,
            _ => true,
        }
    }
}

impl<'a> GroupScoreStringCounter<'a> {
    fn init_scan(&mut self) -> GroupScoreStringCounterState {
        if let Some(c) = self.reference_string.chars().nth(0) {
            match c {
                '{' => return GroupScoreStringCounterState::GroupOpen,
                '<' => {
                    self.pos += 1;
                    return GroupScoreStringCounterState::GarbageSkip;
                }
                _ => return GroupScoreStringCounterState::Done,
            };
        }
        return GroupScoreStringCounterState::Done;
    }
}

impl<'a> GroupScoreStringCounter<'a> {
    fn group_open(&mut self) -> GroupScoreStringCounterState {
        while let Some(c) = self.reference_string.chars().nth(self.pos) {
            match c {
                '!' => self.pos += 1,
                '{' => self.depth += 1,
                '}' => {
                    return GroupScoreStringCounterState::GroupClose;
                }
                '<' => {
                    self.pos += 1;
                    return GroupScoreStringCounterState::GarbageSkip;
                }
                _ => {}
            }
            self.pos += 1;
        }
        return GroupScoreStringCounterState::Done;
    }
}

impl<'a> GroupScoreStringCounter<'a> {
    fn group_close(&mut self) -> GroupScoreStringCounterState {
        while let Some(c) = self.reference_string.chars().nth(self.pos) {
            match c {
                '!' => self.pos += 1,
                '}' => {
                    self.score += self.depth;
                    self.depth -= 1;
                }
                '{' => {
                    return GroupScoreStringCounterState::GroupOpen;
                }
                '<' => {
                    self.pos += 1;
                    return GroupScoreStringCounterState::GarbageSkip;
                }
                _ => {}
            }
            self.pos += 1
        }
        return GroupScoreStringCounterState::Done;
    }
}

impl<'a> GroupScoreStringCounter<'a> {
    fn garbage_skip(&mut self) -> GroupScoreStringCounterState {
        while let Some(c) = self.reference_string.chars().nth(self.pos) {
            match c {
                '!' => self.pos += 1,
                '>' => {
                    self.pos += 1;
                    return GroupScoreStringCounterState::Init;
                }
                '<' => {
                    self.garbage_count += 1;
                }
                _ => {
                    self.garbage_count += 1;
                }
            }
            self.pos += 1;
        }
        return GroupScoreStringCounterState::Done;
    }
}
