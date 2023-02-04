pub mod input_buffer;
pub mod meta_command;
pub mod page;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;