use std::fs;
use crate::Row;
use crate::editor::Position;
use std::io::{Write, Error};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
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
        })
    }

    pub fn insert(&mut self, pos: &Position, c: char) {
        if pos.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else if pos.y < self.len() {
            let row = self.rows.get_mut(pos.y).unwrap();
            row.insert(pos.x, c);
        }
    }

    pub fn delete(&mut self, pos: &Position) {
        if pos.y >= self.len() {
            return;
        } 
        
        if pos.x == self.rows.get_mut(pos.y).unwrap().len() && pos.y < self.len() - 1 {
            let next_row = self.rows.remove(pos.y + 1);
            let current_row = self.rows.get_mut(pos.y).unwrap();
            current_row.append(&next_row);
        } else {
            let row = self.rows.get_mut(pos.y).unwrap();
            row.delete(pos.x);
        }
    }

    pub fn new_line(&mut self, pos: &Position) {
        if pos.y > self.len() {
            return;
        }
        if pos.y == self.len() {
            self.rows.push(Row::default());
            return;
        } else {
            let new_row = self.rows.get_mut(pos.y).unwrap().split(pos.x);
            self.rows.insert(pos.y + 1, new_row)
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        if let Some(name) = &self.filename {
            let mut file = fs::File::create(name)?;
            for row in &self.rows {
                //don't really wanna add a newline to the last row, should fix
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }

        Ok(())
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
}