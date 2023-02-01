use crate::{Error, Result};
use std::io::Write;

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
        self.buffer = String::new();
        self.buffer_length = 0;
        self.input_length = 0;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        self.buffer = input;
        self.buffer_length = self.buffer.len();
        self.input_length = self.buffer.trim_end().len();
        Ok(())
    }

    fn close_input_buffer(&mut self) {
        self.buffer = String::new();
        self.buffer_length = 0;
        self.input_length = 0;
    }

    pub fn get_buffer(&self) -> &str {
        &self.buffer[..self.input_length]
    }
}

impl Drop for InputBuffer {
    fn drop(&mut self) {
        self.close_input_buffer();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_buffer() {
        let mut input_buffer = InputBuffer::new();
        input_buffer.print_prompt();
        input_buffer.read_input().unwrap();
        input_buffer.close_input_buffer();
        assert_eq!(input_buffer.buffer, "db > ");
    }
}
