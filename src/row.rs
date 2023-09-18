use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) ->Self {
        let mut row = Self { 
            string: String::from(slice),
            len: 0,
        };
        row.update_len();
        row
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
        } else {
            let mut result: String = self.string[..].graphemes(true).take(index).collect();
            let remainder: String = self.string[..].graphemes(true).skip(index).collect();
            result.push(c);
            result.push_str(&remainder);
            self.string = result
        }
        self.update_len()
    }

    #[allow(clippy::arithmetic_side_effects)]
    pub fn delete(&mut self, index: usize) {
        if index >= self.len() {
            return;
        } else {
            let mut result: String = self.string[..].graphemes(true).take(index).collect();
            let remainder: String = self.string[..].graphemes(true).skip(index + 1).collect();
            result.push_str(&remainder);
            self.string = result;
        }

        self.update_len()
    }

    pub fn split(&mut self, index: usize) -> Self {
        let beginning: String = self.string[..].graphemes(true).take(index).collect();
        let remainder: String = self.string[..].graphemes(true).skip(index).collect();
        self.string = beginning;
        self.update_len();
        Self::from(&remainder[..])
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn append(&mut self, other: &Self) {
        self.string.push_str(&other.string);
        self.update_len()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }
}