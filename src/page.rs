use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::mem::size_of;

use crate::meta_command::CommandError;
use crate::meta_command::Statement;

type Result<T> = std::result::Result<T, CommandError>;

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

#[derive(Debug)]
pub struct Cursor<'a> {
    table: &'a mut Table,
    row_num: usize,
    end_of_table: bool,
}

impl<'a> Cursor<'a> {
    pub fn table_start(table: &'a mut Table) -> Self {
        let end_of_table = table.num_rows() == 0;
        Self {
            table,
            row_num: 0,
            end_of_table,
        }
    }

    pub fn table_end(table: &'a mut Table) -> Self {
        let row_num = table.num_rows();
        Self {
            table,
            row_num,
            end_of_table: true,
        }
    }

    pub fn cursor_value(&mut self) -> (usize, usize) {
        let row_num = self.row_num;
        let page_num = row_num / ROWS_PER_PAGE;
        self.table.pager.get_page(page_num);
        let row_offset = row_num % ROWS_PER_PAGE;
        (page_num, row_offset)
    }

    pub fn cursor_advance(&mut self) {
        self.row_num += 1;
        if self.row_num >= self.table.num_rows() {
            self.end_of_table = true;
        }
    }

    pub fn is_end_of_table(&self) -> bool {
        self.end_of_table
    }

    pub fn get_row(&mut self, page_num: usize, row_num: usize) -> Result<Row> {
        let row = self.table.pager.get_row(page_num, row_num);
        Ok(row)
    }

    pub fn flush(&mut self, page_num: usize, row_num: usize) {
        self.table.pager.flush(page_num, row_num);
    }
}

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

#[derive(Debug)]
struct Pager {
    file: std::fs::File,
    file_length: usize,
    pages: [Option<[u8; PAGE_SIZE]>; TABLE_MAX_PAGES],
}

impl Pager {
    pub fn new() -> Self {
        Self {
            file: std::fs::File::create("db").unwrap(),
            file_length: 0,
            pages: [None; TABLE_MAX_PAGES],
        }
    }
    pub fn open(file_path: &str) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();
        // TODO: handle file not found
        let file_length = file.metadata().unwrap().len() as usize;
        let mut num_pages = file_length / PAGE_SIZE;
        if file_length % PAGE_SIZE > 0 {
            num_pages += 1;
        }
        let mut pages = [None; TABLE_MAX_PAGES];
        for i in 0..num_pages {
            let mut buffer = [0; PAGE_SIZE];
            file.read(&mut buffer).unwrap();
            pages[i] = Some(buffer);
        }
        Self {
            file,
            file_length,
            pages,
        }
    }

    pub fn get_file_length(&self) -> usize {
        self.file_length
    }

    pub fn get_page(&mut self, page_num: usize) -> Option<&[u8; PAGE_SIZE]> {
        // TODO: handle page_num out of bounds
        if page_num > TABLE_MAX_PAGES {
            panic!(
                "Tried to fetch page number out of bounds. Max page number: {}, got {}",
                TABLE_MAX_PAGES, page_num
            );
        }
        match self.pages[page_num] {
            Some(_) => (),
            None => {
                let mut buffer = [0; PAGE_SIZE];
                self.pages[page_num] = Some(buffer);
            }
        }
        self.pages[page_num].as_ref()
    }

    pub fn flush(&mut self, page_num: usize, row_num: usize) {
        let offset: u64 = page_num as u64 * PAGE_SIZE as u64 + row_num as u64 * ROW_SIZE as u64;
        self.file.set_len(offset);
        let num_full_pages = row_num / ROWS_PER_PAGE;
        let row_offset = row_num % ROWS_PER_PAGE;
        let buffer = self.get_row(page_num, row_num).serialize();
        self.file.write(buffer.as_ref());
    }

    fn page_to_bytes(&self, page_num: usize) -> &[u8; PAGE_SIZE] {
        self.pages[page_num].as_ref().unwrap()
    }

    pub fn get_row(&self, page_num: usize, row_num: usize) -> Row {
        let page = self.page_to_bytes(page_num);
        let offset = row_num * ROW_SIZE;
        let row = &page[offset..offset + ROW_SIZE];
        Row::deserialize(row)
    }

    pub fn set_row(&mut self, page_num: usize, row_num: usize, row: Option<Row>) {
        let page = self.pages[page_num].as_mut().unwrap();
        let offset = row_num * ROW_SIZE;
        if let Some(row) = row {
            page[offset..offset + ROW_SIZE].copy_from_slice(&row.serialize());
        } else {
            page[offset..offset + ROW_SIZE].fill(0);
        }
    }
}

#[derive(Debug)]
pub struct Table {
    pager: Pager,
    num_rows: usize,
}

impl Table {
    pub fn new() -> Self {
        Self {
            pager: Pager::new(),
            num_rows: 0,
        }
    }

    pub fn new_from_file(path: &str) -> Self {
        let pager = Pager::open(path);
        let num_rows = pager.get_file_length() / ROW_SIZE;
        Self { pager, num_rows }
    }


    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn insert(&mut self, statement: &Statement) -> Result<()> {
        if self.num_rows >= TABLE_MAX_ROWS {
            return Err(CommandError::ExecuteTableFull);
        }
        let mut cursor = Cursor::table_start(self);
        let (page_num, row_offset) = cursor.cursor_value();
        self.pager
            .set_row(page_num, row_offset, statement.get_row_to_insert());
        self.num_rows += 1;
        println!(
            "insert successfully {}",
            self.pager.get_row(page_num, row_offset)
        );
        Ok(())
    }

    pub fn select(&mut self, statement: &Statement) -> Result<()> {
        let mut cursor = Cursor::table_start(self);
        while cursor.is_end_of_table() {
            let (page_num, row_offset) = cursor.cursor_value();
            let row = cursor.get_row(page_num, row_offset)?;
            println!("{}", row);
            cursor.cursor_advance();
        }
        Ok(())
    }

    pub fn db_close(&mut self) {
        let mut cursor = Cursor::table_start(self);
        while !cursor.is_end_of_table() {
            let (page_num, row_offset) = cursor.cursor_value();
            cursor.flush(page_num, row_offset);
            cursor.cursor_advance();
        }
    }
}
