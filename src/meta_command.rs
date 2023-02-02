use std::default;
use std::process::Command;

use crate::input_buffer::InputBuffer;
use crate::{Error, Result};

pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

pub enum StatementType {
    StatementInsert,
    StatementSelect,
}

pub struct Statement {
    statement_type: StatementType,
}

pub fn do_meta_command(input_buffer: &mut InputBuffer) -> Result<MetaCommandResult> {
    if input_buffer.get_buffer() == ".exit" {
        std::process::exit(0);
    } else {
        Ok(MetaCommandResult::MetaCommandUnrecognizedCommand)
    }
}

pub fn parse_statement(buffer: &mut InputBuffer) -> Result<Statement> {
    if buffer.get_buffer().starts_with("insert") {
        Ok(Statement {
            statement_type: StatementType::StatementInsert,
        })
    } else if buffer.get_buffer().starts_with("select") {
        Ok(Statement {
            statement_type: StatementType::StatementSelect,
        })
    } else {
        Err("Unrecognized keyword at start of".into())
    }
}

impl Statement {
    pub fn execute(&self) -> Result<()> {
        match &self.statement_type {
            StatementType::StatementInsert => {
                println!("This is where we would do an insert.");
                Ok(())
            }
            StatementType::StatementSelect => {
                println!("This is where we would do a select.");
                Ok(())
            }
            _default => Err("Unrecognized keyword at start of".into()),
        }
    }
}
