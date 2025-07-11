use std::fmt::Debug;

enum State {
    RawRead,
    TokenDecode,
    Done,
}

pub struct Inflator<'a> {
    recursive: bool,
    reference_chars: &'a [char],
    pos: usize,
    char_count: usize,
    state: State,
}

impl<'a> Debug for Inflator<'a> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(_f, "Recursive: {}", self.recursive)?;
        writeln!(_f, "Position: {}", self.pos)?;
        writeln!(_f, "Character Count: {}", self.char_count)?;
        match self.state {
            State::RawRead => writeln!(_f, "State: Raw Read")?,
            State::TokenDecode => writeln!(_f, "State: Token Decode")?,
            State::Done => writeln!(_f, "State: Done")?,
        }
        Ok(())
    }
}

impl<'a> Inflator<'a> {
    pub fn new(reference_chars: &'a [char], recursive: bool) -> Self {
        Self {
            recursive,
            reference_chars,
            pos: 0,
            char_count: 0,
            state: State::RawRead,
        }
    }

    fn scan(&mut self) {
        while self.pos < self.reference_chars.len() {
            match self.state {
                State::RawRead => self.raw_read(),
                State::TokenDecode => self.token_decode().expect("Unable to decode token"),
                State::Done => return,
            }
        }
    }

    pub fn count_inflated(&mut self) -> usize {
        self.scan();
        self.char_count
    }
}

impl Inflator<'_> {

    /// Raw read counds characters until it finds a `(`
    fn raw_read(&mut self) {
        if let Some(pos) = self.reference_chars[self.pos..]
            .iter()
            .position(|c| c == &'(')
        {
            self.char_count += self.reference_chars[self.pos..self.pos + pos].len();
            self.pos += pos;
            self.state = State::TokenDecode;
            return;
        }

        self.char_count += self.reference_chars[self.pos..].len();
        self.pos = self.reference_chars.len() - 1;
        self.state = State::Done;
    }
}

#[derive(Debug)]
struct MalformedBytes;

impl Inflator<'_> {

    /// Token decode decodes a token `(length)x(repetition)`
    /// and then counts `length` characters `repetition` times
    /// if recursive is true, it also inflates any nested tokens
    fn token_decode(&mut self) -> Result<(), MalformedBytes> {
        let end_pos = self.reference_chars[self.pos..]
            .iter()
            .position(|c| c == &')')
            .ok_or(MalformedBytes)?;

        let token_string = self.reference_chars[self.pos + 1..self.pos + end_pos]
            .iter()
            .collect::<String>();
        let (l, r) = token_string.split_once("x").ok_or(MalformedBytes)?;
        let length = l.parse::<usize>().map_err(|_| MalformedBytes)?;
        let repetition = r.parse::<usize>().map_err(|_| MalformedBytes)?;

        if self.recursive {
            let sub_pos = self.pos + token_string.len() + 2;
            let sub = &self.reference_chars[sub_pos..sub_pos + length];
            let mut inf = Inflator::new(sub, true);
            self.char_count += repetition * inf.count_inflated();
        } else {
            self.char_count += length * repetition;
        }
        self.pos += token_string.len() + length + 2;
        self.state = State::RawRead;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_inflator {
        ($s:expr,$l:expr,$r:expr) => {
            let chars = $s.chars().into_iter().collect::<Vec<_>>();
            let mut inflator = Inflator::new(&chars.as_slice(), $r);
            assert_eq!(inflator.count_inflated(), $l);
        };
    }

    #[test]
    fn can_count_inflated_chars() {
        assert_inflator!("ADVENT", 6, false);
        assert_inflator!("A(1x5)BC", 7, false);
        assert_inflator!("(3x3)XYZ", 9, false);
        assert_inflator!("A(2x2)BCD(2x2)EFG", 11, false);
        assert_inflator!("(6x1)(1x3)A", 6, false);
        assert_inflator!("X(8x2)(3x3)ABCY", 18, false);
    }

    #[test]
    fn can_count_inflated_chars_recursively() {
        assert_inflator!("(3x3)XYZ", 9, true);
        assert_inflator!("X(8x2)(3x3)ABCY", 20, true);
        assert_inflator!("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920, true);
        assert_inflator!(
            "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
            445,
            true
        );
    }
}
