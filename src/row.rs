use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    #[must_use]
    #[allow(clippy::string_slice)]
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        #[allow(clippy::integer_arithmetic)]
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push(' ');
            } else {
                result.push_str(grapheme);
            }
        }

        result
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[allow(clippy::integer_arithmetic, clippy::string_slice)]
    pub fn insert(&mut self, pos: usize, c: char) {
        if pos >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if index == pos {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.string = result;
    }

    #[allow(clippy::integer_arithmetic, clippy::string_slice)]
    pub fn delete(&mut self, pos: usize) {
        if pos >= self.len() {
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != pos {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.string = result;
    }

    #[allow(clippy::integer_arithmetic)]
    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    #[must_use]
    #[allow(clippy::integer_arithmetic, clippy::string_slice)]
    pub fn split(&mut self, pos: usize) -> Self {
        let mut row: String = String::new();
        let mut length = 0;
        let mut split_row: String = String::new();
        let mut split_length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < pos {
                length += 1;
                row.push_str(grapheme);
            } else {
                split_length += 1;
                split_row.push_str(grapheme);
            }
        }

        self.string = row;
        self.len = length;
        Self {
            string: split_row,
            len: split_length,
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn find(&self, query: &str, after: usize) -> Option<usize> {
        let substring: String = self.string[..].graphemes(true).skip(after).collect();
        let matching_byte_index = substring.find(query);
        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in substring[..].grapheme_indices(true).enumerate() {
                if matching_byte_index == byte_index {
                    return Some(after + grapheme_index);
                }
            }
        }
        None
    }
}