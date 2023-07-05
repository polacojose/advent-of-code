const MAX_BUFFER_SIZE: usize = 10240;

pub struct EasterBunnyInflator {
    pub reference_chars: Vec<char>,
    pub pos: usize,
    state: EasterBunnyInflatorState,
}

impl EasterBunnyInflator {
    pub fn new(line: impl AsRef<str>) -> Self {
        let chars = line.as_ref().chars().collect::<Vec<_>>();
        EasterBunnyInflator {
            reference_chars: chars,
            pos: 0,
            state: EasterBunnyInflatorState::RawCopy,
        }
    }

    fn new_chars(chars: Vec<char>) -> Self {
        EasterBunnyInflator {
            reference_chars: chars,
            pos: 0,
            state: EasterBunnyInflatorState::RawCopy,
        }
    }
}

enum EasterBunnyInflatorState {
    RawCopy,
    TokenDecode,
    TokenExecute {
        characters: usize,
        iterations: usize,
    },
    Done,
}

impl Iterator for EasterBunnyInflator {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let result = match &self.state {
                EasterBunnyInflatorState::RawCopy => self.raw_copy(),
                EasterBunnyInflatorState::TokenDecode => self.token_decode(),
                EasterBunnyInflatorState::TokenExecute {
                    characters: _,
                    iterations: _,
                } => self.token_execute(),
                EasterBunnyInflatorState::Done => None,
            };

            match &result {
                Some(contents) => {
                    if contents.len() > 0 {
                        return result;
                    }
                }
                None => return None,
            }
        }
    }
}

impl EasterBunnyInflator {
    fn raw_copy(&mut self) -> Option<Vec<char>> {
        match &self.state {
            EasterBunnyInflatorState::RawCopy => {
                if self.pos >= self.reference_chars.len() {
                    self.state = EasterBunnyInflatorState::Done;
                    return None;
                }

                let start_buffer = self.pos;
                let mut end_buffer =
                    (start_buffer + MAX_BUFFER_SIZE).min(self.reference_chars.len());

                if let Some(position) = self
                    .reference_chars
                    .iter()
                    .enumerate()
                    .position(|(i, x)| i >= start_buffer && i < end_buffer && x == &'(')
                {
                    end_buffer = end_buffer.min(position);
                    self.state = EasterBunnyInflatorState::TokenDecode;
                }

                self.pos = end_buffer;

                return Some(self.reference_chars[start_buffer..end_buffer].to_vec());
            }
            _ => {}
        }
        return Some(vec![]);
    }
}

impl EasterBunnyInflator {
    fn token_decode(&mut self) -> Option<Vec<char>> {
        match &mut self.state {
            EasterBunnyInflatorState::TokenDecode {} => {
                self.pos += 1;

                let end_token_position = self
                    .reference_chars
                    .iter()
                    .skip(self.pos)
                    .position(|x| x == &')')
                    .unwrap()
                    + self.pos;

                let token_str = String::from_iter(
                    self.reference_chars[self.pos..end_token_position].into_iter(),
                );

                let (characters_string, iterations_string) = token_str.split_once("x").unwrap();

                self.state = EasterBunnyInflatorState::TokenExecute {
                    characters: characters_string.parse().unwrap(),
                    iterations: iterations_string.parse().unwrap(),
                };

                self.pos = end_token_position + 1;
            }
            _ => {}
        }
        return Some(vec![]);
    }
}

impl EasterBunnyInflator {
    fn token_execute(&mut self) -> Option<Vec<char>> {
        match self.state {
            EasterBunnyInflatorState::TokenExecute {
                characters,
                iterations,
            } => {
                let chars = self.reference_chars[self.pos..(self.pos + characters)].as_ref();

                let mut c: Vec<char> = Vec::with_capacity(characters * iterations);
                for _ in 0..iterations {
                    c.extend_from_slice(chars);
                }

                self.pos += characters;

                if let Some(char) = self.reference_chars.get(self.pos) {
                    if char == &'(' {
                        self.state = EasterBunnyInflatorState::TokenDecode;
                        return Some(c);
                    }
                }

                self.state = EasterBunnyInflatorState::RawCopy;
                return Some(c);
            }
            _ => {}
        }
        return Some(vec![]);
    }
}

pub struct EasterBunnyRecursiveInflator {
    inflator: EasterBunnyInflator,
}

impl EasterBunnyRecursiveInflator {
    pub fn new(line: impl AsRef<str>) -> Self {
        let chars: Vec<_> = line.as_ref().chars().collect();
        Self {
            inflator: EasterBunnyInflator {
                reference_chars: chars,
                pos: 0,
                state: EasterBunnyInflatorState::RawCopy,
            },
        }
    }
}

impl Iterator for EasterBunnyRecursiveInflator {
    type Item = Vec<char>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let result = self.inflator.next();
            match result {
                Some(mut chars) => {
                    if chars.contains(&'(') {
                        let position = self.inflator.pos;
                        chars.extend_from_slice(self.inflator.reference_chars[position..].as_ref());

                        self.inflator = EasterBunnyInflator::new_chars(chars);
                        self.inflator.state = EasterBunnyInflatorState::TokenDecode;
                    } else {
                        return Some(chars);
                    }
                }
                None => return None,
            }
        }
    }
}
