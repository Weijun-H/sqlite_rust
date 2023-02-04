use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::size_of;

use crate::meta_command::ExecuteResult;
use crate::meta_command::Statement;
use crate::{Error, Result};

pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

const ID_SIZE: usize = size_of::<u32>();
const USERNAME_SIZE: usize = COLUMN_USERNAME_SIZE;
const EMAIL_SIZE: usize = COLUMN_EMAIL_SIZE;

const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

#[derive(Debug, Copy, Clone)]
pub struct Row {
    pub id: u32,
    pub username: [u8; COLUMN_USERNAME_SIZE],
    pub email: [u8; COLUMN_EMAIL_SIZE],
}

impl Row {
    pub fn new(
        id: u32,
        username: [u8; COLUMN_USERNAME_SIZE],
        email: [u8; COLUMN_EMAIL_SIZE],
    ) -> Self {
        Self {
            id,
            username,
            email,
        }
    }

    pub fn serialize(&self) -> [u8; ROW_SIZE] {
        let mut buffer = [0; ROW_SIZE];
        let mut offset = 0;

        let id = self.id.to_le_bytes();
        buffer[offset..offset + ID_SIZE].copy_from_slice(&id);
        offset += ID_SIZE;

        buffer[offset..offset + USERNAME_SIZE].copy_from_slice(&self.username);
        offset += USERNAME_SIZE;

        buffer[offset..offset + EMAIL_SIZE].copy_from_slice(&self.email);

        buffer
    }

    pub fn deserialize(buffer: &[u8]) -> Self {
        let mut offset = 0;

        let id = u32::from_le_bytes([
            buffer[offset],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3],
        ]);
        offset += ID_SIZE;

        let mut username = [0; COLUMN_USERNAME_SIZE];
        username[..].copy_from_slice(&buffer[offset..offset + USERNAME_SIZE]);
        offset += USERNAME_SIZE;

        let mut email = [0; COLUMN_EMAIL_SIZE];
        email[..].copy_from_slice(&buffer[offset..offset + EMAIL_SIZE]);

        Self {
            id,
            username,
            email,
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.id,
            String::from_utf8_lossy(&self.username),
            String::from_utf8_lossy(&self.email)
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Table {
    pages: [Option<[Option<Row>; ROWS_PER_PAGE]>; TABLE_MAX_PAGES],
    num_rows: usize,
}

impl Table {
    pub fn new() -> Self {
        Self {
            pages: [None; TABLE_MAX_PAGES],
            num_rows: 0,
        }
    }

    pub fn row_slot(&mut self, row_num: usize) -> (usize, usize) {
        let page_num = row_num / ROWS_PER_PAGE;
        let row_offset = row_num % ROWS_PER_PAGE;
        match self.pages[page_num] {
            Some(_) => (),
            None => self.pages[page_num] = Some([None; ROWS_PER_PAGE]),
        }
        (page_num, row_offset)
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn insert(&mut self, statement: &Statement) -> Result<ExecuteResult> {
        if self.num_rows >= TABLE_MAX_ROWS {
            return Err("Table full".into());
            // Err(ExecuteResult::ExecuteTableFull)
        }
        let (page_num, row_offset) = self.row_slot(self.num_rows);
        self.pages[page_num].as_mut().unwrap()[row_offset] = statement.get_row_to_insert();
        self.num_rows += 1;
        println!(
            "insert successfully {}",
            self.pages[page_num].unwrap()[row_offset].unwrap()
        );
        Ok(ExecuteResult::ExecuteSuccess)
    }

    pub fn select(&mut self, statement: &Statement) -> Result<ExecuteResult> {
        for i in 0..self.num_rows {
            let (page_num, row_offset) = self.row_slot(i);
            let row = self.pages[page_num].unwrap()[row_offset].unwrap();
            println!("{}", row);
        }
        Ok(ExecuteResult::ExecuteSuccess)
    }
}
