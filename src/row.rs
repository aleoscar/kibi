use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) ->Self {
        Self { 
            string: String::from(slice),
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        #[allow(clippy::arithmetic_side_effects)]
        for grapheme in self.string
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            //replaces tabs with a space
            if grapheme == "\t" {
                result.push_str(" ");
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }

    pub fn insert(&mut self, index: usize, c: char) {
        if index >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (i, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if i == index {
                length += 1;
                result.push(c)
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.string = result;
    }

    #[allow(clippy::arithmetic_side_effects)]
    pub fn delete(&mut self, index: usize) {
        if index >= self.len() {
            return;
        } 
        let mut result: String = String::new();
        let mut length = 0;
        for (i, grapheme) in self.string[..].graphemes(true).enumerate() {
            if i != index {
                length += 1;
                result.push_str(grapheme)
            }
        }
        self.len = length;
        self.string = result;
    }

    pub fn split(&mut self, index: usize) -> Self {
        let mut row: String = String::new();
        let mut length = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length = 0;

        for (i, grapheme) in self.string.graphemes(true).enumerate() {
            if i < index {
                length += 1;
                row.push_str(grapheme)
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }

        self.string = row;
        self.len = length;

        Self { string: splitted_row, len: splitted_length }
    }

    pub fn is_alphanumeric(&self, index: usize) -> bool {
        self.string.chars().nth(index).unwrap().is_alphanumeric()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn append(&mut self, other: &Self) {
        self.string.push_str(&other.string);
        self.len += other.len;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
