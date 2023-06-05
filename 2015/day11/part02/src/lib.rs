use std::{borrow::BorrowMut, str::FromStr};

use regex::Regex;

#[derive(Debug)]
pub struct AlphaNum {
    alpha: String,
}

#[derive(Debug)]
pub struct UnableToParse;

impl AlphaNum {
    pub fn increment(&mut self) {
        let chars = self.alpha.chars().collect::<Vec<char>>();

        let mut new_string = String::new();

        let mut carry = true;
        let mut i: i32 = chars.len() as i32 - 1;
        while i >= 0 || carry == true {
            let c = chars.get(i as usize);
            if carry {
                carry = false;
                if let Some(c) = c {
                    if c == &'z' {
                        new_string.insert(0, 'a');
                        carry = true;
                    } else {
                        new_string.insert(0, (*c as u8 + 1) as char);
                    }
                } else {
                    new_string.insert(0, 'b');
                }
            } else {
                if let Some(c) = c {
                    new_string.insert(0, *c);
                }
            }

            i -= 1;
        }

        while new_string.as_bytes().len() < 8 {
            new_string.insert(0, 'a');
        }

        self.alpha = new_string;
    }

    pub fn decrement(&mut self) {
        let chars = self.alpha.chars().collect::<Vec<char>>();

        let mut new_string = String::new();

        let mut carry = true;
        let mut i: i32 = chars.len() as i32 - 1;
        while i >= 0 || carry == true {
            let c = chars.get(i as usize);
            if carry {
                carry = false;
                if let Some(c) = c {
                    if c == &'a' {
                        new_string.insert(0, 'z');
                        carry = true;
                    } else {
                        new_string.insert(0, (*c as u8 - 1) as char);
                    }
                }
            } else {
                if let Some(c) = c {
                    new_string.insert(0, *c);
                }
            }

            i -= 1;
        }

        while new_string.as_bytes().len() < 8 {
            new_string.insert(0, 'a');
        }

        self.alpha = new_string;
    }

    pub fn get_alpha(&self) -> &str {
        &self.alpha
    }
}

impl FromStr for AlphaNum {
    type Err = UnableToParse;

    fn from_str(alpha: &str) -> Result<Self, Self::Err> {
        let mut alpha = alpha.trim().to_lowercase();
        let re = Regex::new(r"^[a-z]+$").unwrap();

        if !re.is_match(alpha.as_str()) {
            return Err(UnableToParse);
        }

        while alpha.as_bytes().len() < 8 {
            alpha.insert(0, 'a');
        }

        Ok(Self { alpha })
    }
}

pub mod password_rules {
    pub fn valid_password(password: &str) -> bool {
        if includes_three_straight_letters(password)
            && !includes_invalid_characters(password)
            && includes_two_pairs(password)
        {
            return true;
        }
        false
    }
    fn includes_three_straight_letters(password: &str) -> bool {
        let chars = password.chars().collect::<Vec<char>>();
        if chars.len() < 3 {
            return false;
        }

        for window in chars.windows(3) {
            if (window[0] as u8 + 2) == (window[1] as u8 + 1)
                && (window[1] as u8 + 1) == (window[2] as u8)
            {
                return true;
            }
        }

        return false;
    }

    fn includes_invalid_characters(password: &str) -> bool {
        if password.contains('i') || password.contains('o') || password.contains('l') {
            return true;
        }
        false
    }

    fn includes_two_pairs(password: &str) -> bool {
        let chars = password.chars().collect::<Vec<char>>();
        if chars.len() < 4 {
            return false;
        }

        let mut num_pairs = 0;

        let mut i = 1;
        while i < chars.len() {
            if chars[i] == chars[i - 1] {
                num_pairs += 1;
                i += 1;
            }

            if num_pairs == 2 {
                return true;
            }

            i += 1;
        }

        return false;
    }
}
