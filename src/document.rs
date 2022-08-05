use super::Position;
use super::Row;

use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    fn insert_newline(&mut self, pos: &Position) {
        if pos.y > self.rows.len() {
            return;
        }
        if pos.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }
        #[allow(clippy::indexing_slicing)]
        let new_row = self.rows[pos.y].split(pos.x);
        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(pos.y + 1, new_row);
    }

    pub fn insert(&mut self, pos: &Position, c: char) {
        if pos.y > self.rows.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(pos);
            return;
        }
        if pos.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[pos.y];
            row.insert(pos.x, c);
        }
    }

    #[allow(clippy::integer_arithmetic, clippy::indexing_slicing)]
    pub fn delete(&mut self, pos: &Position) {
        let len = self.rows.len();
        if pos.y >= len {
            return;
        }
        self.dirty = true;
        if pos.x == self.rows[pos.y].len() && pos.y + 1 < len {
            let next_row = self.rows.remove(pos.y + 1);
            let row = &mut self.rows[pos.y];
            row.append(&next_row);
        } else {
            let row = &mut self.rows[pos.y];
            row.delete(pos.x);
        }
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;

            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }

        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn find(&self, query: &str, after: &Position) -> Option<Position> {
        let mut x = after.x;
        for (y, row) in self.rows.iter().enumerate().skip(after.y) {
            if let Some(x) = row.find(query, x) {
                return Some(Position { x, y });
            }
            x = 0;
        }
        None
    }
}