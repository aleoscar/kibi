use std::fs;
use crate::Row;
use crate::editor::Position;
use std::io::{Write, Error};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
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
            filename: Some(filename.to_string()),
            dirty: false,
        })
    }

    pub fn insert(&mut self, pos: &Position, c: char) {
        if pos.y > self.rows.len() {
            return;
        }

        self.dirty = true;
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

    #[allow(clippy::arithmetic_side_effects, clippy::indexing_slicing)]
    pub fn delete(&mut self, pos: &Position) {
        if pos.y >= self.len() {
            return;
        } 

        self.dirty = true;
        
        if pos.x == self.rows[pos.y].len() && pos.y + 1 < self.rows.len() {
            let next_row = self.rows.remove(pos.y + 1);
            let current_row = &mut self.rows[pos.y];
            current_row.append(&next_row);
        } else {
            let row = &mut self.rows[pos.y];
            row.delete(pos.x);
        }
    }

    pub fn new_line(&mut self, pos: &Position) {
        if pos.y > self.rows.len() {
            return;
        }
        if pos.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        } else {
            #[allow(clippy::indexing_slicing)]
            let new_row = self.rows[pos.y].split(pos.x);
            #[allow(clippy::arithmetic_side_effects)]
            self.rows.insert(pos.y + 1, new_row)
        }
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(name) = &self.filename {
            let mut file = fs::File::create(name)?;
            for (i, row) in self.rows.iter().enumerate() {
                file.write_all(row.as_bytes())?;
                if i != self.rows.len() - 1 {
                    file.write_all(b"\n")?;
                }
            }
            self.dirty = false;
        }

        Ok(())
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}