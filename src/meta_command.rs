use std::default;
use std::process::Command;

use crate::input_buffer::InputBuffer;
use crate::page::{Row, Table, COLUMN_EMAIL_SIZE, COLUMN_USERNAME_SIZE};
use crate::{Error, Result};
use scan_fmt::scan_fmt;

pub enum ExecuteResult {
    ExecuteSuccess,
    ExecuteTableFull,
}

pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}
pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
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

pub fn do_meta_command(input_buffer: &mut InputBuffer) -> Result<MetaCommandResult> {
    if input_buffer.get_buffer() == ".exit" {
        std::process::exit(0);
    } else {
        Ok(MetaCommandResult::MetaCommandUnrecognizedCommand)
    }
}

pub fn parse_statement(buffer: &mut InputBuffer) -> Result<Statement> {
    let command = buffer.get_buffer();
    let mut username_tmp = [0; COLUMN_USERNAME_SIZE];
    let mut email_tmp = [0; COLUMN_EMAIL_SIZE];
    if command.starts_with("insert") {
        let (id, username, email) = scan_fmt!(command, "insert {} {} {}", u32, String, String)?;
        username_tmp[..username.len()].copy_from_slice(username.as_bytes());
        email_tmp[..email.len()].copy_from_slice(email.as_bytes());
        Ok(Statement {
            statement_type: StatementType::StatementInsert,
            row_to_insert: Some(Row::new(id, username_tmp, email_tmp)),
        })
    } else if buffer.get_buffer().starts_with("select") {
        Ok(Statement {
            statement_type: StatementType::StatementSelect,
            row_to_insert: None,
        })
    } else {
        Err("Could not parse statement".into())
    }
}

pub fn execute_statement(statement: &Statement, table: &mut Table) -> Result<ExecuteResult> {
    match statement.get_statement_type() {
        StatementType::StatementInsert => table.insert(statement),
        StatementType::StatementSelect => {
            table.select(&statement);
            Ok(ExecuteResult::ExecuteSuccess)
        }
    }
}

pub fn execute_query(buffer: &mut InputBuffer, table: &mut Table) -> Result<()> {
    buffer.print_prompt();
    buffer.read_input()?;
    if buffer.get_buffer().starts_with(".") {
        match do_meta_command(buffer) {
            Ok(MetaCommandResult::MetaCommandSuccess) => {
                return Ok(());
            }
            Ok(MetaCommandResult::MetaCommandUnrecognizedCommand) => {
                return Err("Unrecognized command {buffer.get_buffer()}".into());
            }
            Err(e) => {
                return Err(e);
            }
        }
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
    fn test_query() {
        let query = "insert 1 user1 email1";
        let mut buffer = InputBuffer::from_str(query).unwrap();
        let mut table = Table::new();
        execute_query(&mut buffer, &mut table).unwrap();
        let query = "select";
        let mut buffer = InputBuffer::from_str(query).unwrap();
        execute_query(&mut buffer, &mut table).unwrap();

    }
}