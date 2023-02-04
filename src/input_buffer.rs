use crate::{Error, Result};
use std::io::Write;
use std::str::FromStr;

#[derive(Debug)]
pub struct InputBuffer {
    buffer: String,
    buffer_length: usize,
    input_length: usize,
}

impl InputBuffer {
    pub fn new() -> InputBuffer {
        InputBuffer {
            buffer: String::new(),
            buffer_length: 0,
            input_length: 0,
        }
    }

    pub fn print_prompt(&self) {
        print!("db > ");
        std::io::stdout().flush().unwrap();
    }

    pub fn read_input(&mut self) -> Result<()> {
        if self.input_length == 0 {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            self.buffer = input.to_ascii_lowercase();
            self.buffer_length = self.buffer.len();
            self.input_length = self.buffer.trim_end().len();
        }
        Ok(())
    }


    pub fn get_buffer(&self) -> &str {
        &self.buffer[..self.input_length]
    }
}

impl FromStr for InputBuffer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut input_buffer = InputBuffer::new();
        input_buffer.buffer = s.to_string();
        input_buffer.buffer_length = input_buffer.buffer.len();
        input_buffer.input_length = input_buffer.buffer.trim_end().len();
        Ok(input_buffer)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input_buffer_from_str() {
        let input_buffer = InputBuffer::from_str("insert 1 user1 email1").unwrap();
        assert_eq!(input_buffer.buffer, "insert 1 user1 email1");
        assert_eq!(input_buffer.buffer_length, 21);
        assert_eq!(input_buffer.input_length, 21);
    }
}
