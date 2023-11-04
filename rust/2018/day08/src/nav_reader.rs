use std::io::{self, Read};

enum NavFileReaderState {
    Reading,
    EOF,
}

pub struct NavReader<R>
where
    R: Read,
{
    readable: R,
    next_num: Option<u8>,
    state: NavFileReaderState,
}

impl<R> NavReader<R>
where
    R: Read,
{
    pub fn new(readable: R) -> Result<Self, io::Error> {
        Ok(Self {
            readable,
            next_num: None,
            state: NavFileReaderState::Reading,
        })
    }

    fn fill_buffer(&mut self) {
        match &self.state {
            NavFileReaderState::EOF => {
                self.next_num = None;
                return;
            }
            _ => {}
        }

        let mut num_buff: [u8; 2] = [0; 2];
        let mut num_buff_index = 0;
        let mut buff: [u8; 1] = [0; 1];
        while let Ok(count) = self.readable.read(&mut buff) {
            if count == 0 {
                self.state = NavFileReaderState::EOF;
                self.next_num =
                    if let Some(num_str) = std::str::from_utf8(&num_buff[0..num_buff_index]).ok() {
                        num_str.parse().ok()
                    } else {
                        None
                    };
                break;
            }

            if buff[0] == ' ' as u8 || buff[0] == '\n' as u8 {
                self.next_num =
                    if let Some(num_str) = std::str::from_utf8(&num_buff[0..num_buff_index]).ok() {
                        num_str.parse().ok()
                    } else {
                        None
                    };
                break;
            }

            num_buff[num_buff_index] = buff[0];
            num_buff_index += 1;
        }
    }
}

impl<R> Iterator for NavReader<R>
where
    R: Read,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.fill_buffer();
        return self.next_num;
    }
}

#[cfg(test)]
mod tests {

    use crate::string_reader::StringReader;

    use super::*;

    #[test]
    fn can_read() {
        let nav_data = "8 11 12 9 0".to_owned();
        let string_reader = StringReader::new(&nav_data);
        let result = NavReader::new(string_reader)
            .unwrap()
            .into_iter()
            .collect::<Vec<u8>>();
        assert_eq!(result, vec![8, 11, 12, 9, 0]);
    }
}
