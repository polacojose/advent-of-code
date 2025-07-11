use std::{
    io::{self, Read},
    slice::Iter,
};

pub struct StringReader<'a> {
    iter: Iter<'a, u8>,
}

#[cfg(test)]
impl<'a> StringReader<'a> {
    pub fn new(s: &'a String) -> Self {
        Self {
            iter: s.as_bytes().iter(),
        }
    }
}

impl Read for StringReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut count = 0;
        for i in 0..buf.len() {
            if let Some(next) = self.iter.next() {
                buf[i] = *next;
                count += 1;
            } else {
                break;
            }
        }
        Ok(count)
    }
}
