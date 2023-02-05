use std::default;
use std::fmt;
use std::process::Command;

use crate::input_buffer::InputBuffer;
use crate::page::{Row, Table, COLUMN_EMAIL_SIZE, COLUMN_USERNAME_SIZE};
use scan_fmt::scan_fmt;

type Result<T> = std::result::Result<T, CommandError>;

#[derive(Debug)]
pub enum CommandError {
    // PrepareSuccess,
    PrepareSyntaxError,
    PrepareStringTooLong,
    PrepareNegativeId,
    PrepareUnrecognizedStatement,
    ExecuteTableFull,
    MetaCommandUnrecognizedCommand,
    BufferInputError,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::PrepareSyntaxError => {
                write!(f, "Syntax error. Could not parse statement.")
            }
            CommandError::PrepareStringTooLong => write!(f, "String is too long."),
            CommandError::PrepareNegativeId => write!(f, "ID must be positive."),
            CommandError::PrepareUnrecognizedStatement => {
                write!(f, "Unrecognized keyword at start of '{}'.", "statement")
            }
            CommandError::ExecuteTableFull => write!(f, "Error: Table full."),
            CommandError::MetaCommandUnrecognizedCommand => {
                write!(f, "Unrecognized command '{}'.", "command")
            }
            CommandError::BufferInputError => write!(f, "Error reading input."),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StatementType {
    StatementInsert,
    StatementSelect,
}

pub struct Statement {
    statement_type: StatementType,
    row_to_insert: Option<Row>,
}

impl Statement {
    pub fn new(statement_type: StatementType, row_to_insert: Row) -> Self {
        Self {
            statement_type,
            row_to_insert: Some(row_to_insert),
        }
    }

    pub fn get_statement_type(&self) -> StatementType {
        self.statement_type
    }

    pub fn get_row_to_insert(&self) -> Option<Row> {
        self.row_to_insert
    }
}

pub fn do_meta_command(input_buffer: &mut InputBuffer) -> Result<()> {
    if input_buffer.get_buffer() == ".exit" {
        std::process::exit(0);
    } else {
        Err(CommandError::MetaCommandUnrecognizedCommand)
    }
}

pub fn parse_statement(buffer: &mut InputBuffer) -> Result<Statement> {
    let command = buffer.get_buffer();
    let mut username_tmp = [0; COLUMN_USERNAME_SIZE];
    let mut email_tmp = [0; COLUMN_EMAIL_SIZE];
    if command.starts_with("insert") {
        let scan = scan_fmt!(command, "insert {} {} {}", u32, String, String);
        match scan {
            Ok((id, username, email)) => {
                if id < 0 {
                    return Err(CommandError::PrepareNegativeId);
                }
                if username.len() > COLUMN_USERNAME_SIZE || email.len() > COLUMN_EMAIL_SIZE {
                    return Err(CommandError::PrepareStringTooLong);
                }
                username_tmp[..username.len()].copy_from_slice(username.as_bytes());
                email_tmp[..email.len()].copy_from_slice(email.as_bytes());
                Ok(Statement {
                    statement_type: StatementType::StatementInsert,
                    row_to_insert: Some(Row::new(id, username_tmp, email_tmp)),
                })
            }
            Err(_) => Err(CommandError::PrepareSyntaxError),
        }
    } else if buffer.get_buffer().starts_with("select") {
        Ok(Statement {
            statement_type: StatementType::StatementSelect,
            row_to_insert: None,
        })
    } else {
        Err(CommandError::PrepareUnrecognizedStatement)
    }
}

pub fn execute_statement(statement: &Statement, table: &mut Table) -> Result<()> {
    match statement.get_statement_type() {
        StatementType::StatementInsert => table.insert(statement),
        StatementType::StatementSelect => table.select(&statement),
    }
}

pub fn execute_query(buffer: &mut InputBuffer, table: &mut Table) -> Result<()> {
    buffer.print_prompt();

    match buffer.read_input() {
        Ok(_) => {}
        Err(_) => return Err(CommandError::BufferInputError),
    }
    if buffer.get_buffer().starts_with(".") {
        return do_meta_command(buffer);
    }

    match parse_statement(buffer) {
        Ok(statement) => {
            execute_statement(&statement, table);
            Ok(())
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_query_setup() {
        let query_list = ["insert 1 user1 email1", "select", ".exit"];
        let mut table = Table::new();
        for query in query_list.iter() {
            let mut buffer = InputBuffer::from_str(query).unwrap();
            execute_query(&mut buffer, &mut table).unwrap();
        }
    }

    #[test]
    fn test_query_insert() {
        let mut table = Table::new();
        for i in 1..1000 {
            let query = format!("insert {} user{} email{}", i, i, i);
            let mut buffer = InputBuffer::from_str(query.as_str()).unwrap();
            execute_query(&mut buffer, &mut table).unwrap();
        }
    }

    #[test]
    fn test_query_insert_maximum() {
        let long_username = "a".repeat(COLUMN_USERNAME_SIZE + 1);
        let long_email = "a".repeat(COLUMN_EMAIL_SIZE + 1);
        let query = format!(
            "insert {} {} {}",
            1,
            long_username.as_str(),
            long_email.as_str()
        );
        let mut buffer = InputBuffer::from_str(query.as_str()).unwrap();
        let mut table = Table::new();
        execute_query(&mut buffer, &mut table);
    }
}
