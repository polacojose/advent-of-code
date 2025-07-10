use std::{
    io::{self, Read},
    str::{self, FromStr},
};

use regex::Regex;

#[derive(Debug)]
pub struct MulInstruction {
    op_a: i64,
    op_b: i64,
}

impl MulInstruction {
    pub fn solve(&self) -> i64 {
        self.op_a.saturating_mul(self.op_b)
    }
}

impl FromStr for MulInstruction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        re.captures_iter(s)
            .map(|c| c.extract())
            .map(|(_, [op_a, op_b])| MulInstruction {
                op_a: op_a.parse().unwrap(),
                op_b: op_b.parse().unwrap(),
            })
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid MulInstruction"))
    }
}

enum ReaderState {
    Reading,
    EOF,
}

enum EnableState {
    Do,
    Dont,
}

#[derive(Debug)]
enum Token {
    Do(),
    Dont(),
    Mul(MulInstruction),
}

enum TokenRegex {
    Do(&'static str),
    Dont(&'static str),
    Mul(&'static str),
}

const MUL_REGEX_STR: &str = r"mul\((-?[0-9]+),(-?[0-9]+)\)";

const TOKEN_REGEX: [TokenRegex; 3] = [
    TokenRegex::Do(r"do\(\)"),
    TokenRegex::Dont(r"don't\(\)"),
    TokenRegex::Mul(MUL_REGEX_STR),
];

pub struct MulReader<R>
where
    R: Read,
{
    reader_state: ReaderState,
    read_buffer: Vec<u8>,

    enable_mul_read_state: EnableState,
    readable: R,
    ignore_donts: bool,
    next_mul: Option<MulInstruction>,
}

impl<R> MulReader<R>
where
    R: Read,
{
    pub fn new(readable: R, ignore_donts: bool) -> Self {
        Self {
            reader_state: ReaderState::Reading,
            read_buffer: vec![],

            enable_mul_read_state: EnableState::Do,
            readable,
            ignore_donts,
            next_mul: None,
        }
    }

    pub fn read_next_token(&mut self) {
        match &self.reader_state {
            ReaderState::EOF => {
                self.next_mul = None;
                return;
            }
            _ => {}
        }

        //Find next token
        loop {
            while let Some((tok, pos)) = self.get_next_token_in_buffer() {
                match tok {
                    Token::Do() => {
                        self.enable_mul_read_state = EnableState::Do;
                    }
                    Token::Dont() => {
                        self.enable_mul_read_state = EnableState::Dont;
                    }
                    Token::Mul(s) => {
                        if self.ignore_donts
                            || matches!(self.enable_mul_read_state, EnableState::Do)
                        {
                            self.next_mul = Some(s);
                        }
                    }
                }

                self.read_buffer.drain(..pos + 1);

                if self.next_mul.is_some() {
                    return;
                }
            }

            let mut tmp_buf = [0; 1024];
            if let Ok(n) = self.readable.read(&mut tmp_buf) {
                if n == 0 {
                    self.reader_state = ReaderState::EOF;
                    break;
                }
                self.read_buffer.extend_from_slice(&tmp_buf);
            } else {
                break;
            }
        }
    }

    fn get_next_token_in_buffer(&self) -> Option<(Token, usize)> {
        TOKEN_REGEX
            .iter()
            .filter_map(|tok| {
                let re = match tok {
                    TokenRegex::Do(s) => Regex::new(s).ok()?,
                    TokenRegex::Dont(s) => Regex::new(s).ok()?,
                    TokenRegex::Mul(s) => Regex::new(s).ok()?,
                };

                let m = re.find(str::from_utf8(&self.read_buffer).ok()?)?;
                let token = match tok {
                    TokenRegex::Do(_) => Token::Do(),
                    TokenRegex::Dont(_) => Token::Dont(),
                    TokenRegex::Mul(_) => Token::Mul(m.as_str().parse::<MulInstruction>().ok()?),
                };
                Some((token, m.start()))
            })
            .min_by_key(|t| t.1)
    }
}

impl<R> Iterator for MulReader<R>
where
    R: Read,
{
    type Item = MulInstruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_next_token();
        self.next_mul.take()
    }
}

#[cfg(test)]
mod tests {

    use crate::stringreader::StringReader;

    use super::*;

    const TEST_STR: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_STR_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse_skip() {
        let test_str = TEST_STR.to_owned();
        let string_reader = StringReader::new(&test_str);
        let mul_reader = MulReader::new(string_reader, true);
        let count = mul_reader.into_iter().count();
        assert_eq!(count, 4);
    }

    #[test]
    fn test_parse() {
        let test_str = TEST_STR_2.to_owned();
        let string_reader = StringReader::new(&test_str);
        let mul_reader = MulReader::new(string_reader, false);
        let count = mul_reader.into_iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_mul_skip() {
        let test_str = TEST_STR.to_owned();
        let string_reader = StringReader::new(&test_str);
        let mul_reader = MulReader::new(string_reader, true);
        let sum = mul_reader.into_iter().map(|m| m.solve()).sum::<i64>();
        assert_eq!(sum, 161);
    }

    #[test]
    fn test_mul() {
        let test_str = TEST_STR_2.to_owned();
        let string_reader = StringReader::new(&test_str);
        let mul_reader = MulReader::new(string_reader, false);
        let sum = mul_reader.into_iter().map(|m| m.solve()).sum::<i64>();
        assert_eq!(sum, 48);

        let test_str = TEST_STR_2.to_owned();
        let string_reader = StringReader::new(&test_str);
        let mul_reader = MulReader::new(string_reader, true);
        let sum = mul_reader.into_iter().map(|m| m.solve()).sum::<i64>();
        assert!(sum != 48);
    }
}
