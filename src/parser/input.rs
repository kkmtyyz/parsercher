#[derive(Debug)]
pub struct Input {
    input: Vec<char>,
    cursor: usize,
}

impl Input {
    pub fn new(input: &str) -> Input {
        Input {
            input: input.trim_end().chars().collect(),
            cursor: 0,
        }
    }

    pub fn set_cursor(&mut self, cursor: usize) {
        if self.input.len() <= cursor {
            self.cursor = self.input.len() - 1;
            return;
        }
        self.cursor = cursor;
    }

    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    /// Move the `self.cursor` to the next character.
    pub fn next(&mut self) {
        if self.cursor < self.input.len() - 1 {
            self.cursor += 1;
        }
    }

    /// Move the self.cursor to the next character.
    /// Skip ' ' and '\n'.
    pub fn next_char(&mut self) {
        if self.cursor < self.input.len() - 1 {
            self.cursor += 1;
        }

        let bgn = self.cursor;
        for i in bgn..self.input.len() {
            if self.input[i] == ' ' || self.input[i] == '\n' {
                if self.cursor == self.input.len() - 1 {
                    break;
                }
                self.cursor += 1;
            } else {
                break;
            }
        }
    }

    /// Returns true if the `self.cursor` has reached the end of the `self.input`.
    pub fn is_end(&mut self) -> bool {
        if self.cursor == self.input.len() - 1 {
            return true;
        }
        false
    }

    /// Returns true if the character pointed to by `self.cursor` is equal to `exp`.
    pub fn expect(&self, exp: char) -> bool {
        if self.input[self.cursor] == exp {
            return true;
        } else {
            return false;
        }
    }

    /// Returns true if the string pointed to by the `self.cursor` is equal to `exp`.
    pub fn expect_str(&self, exp: &str) -> bool {
        if self.input.len() < self.cursor + exp.len() {
            return false;
        }

        let cursor = self.cursor;
        let exp: Vec<char> = exp.chars().collect();
        for i in 0..exp.len() {
            if exp[i] != self.input[cursor + i] {
                return false;
            }
        }
        true
    }

    /// Returns true if the string pointed to by the `self.cursor` is equal to `exp`.
    /// case insensitive.
    pub fn expect_str_insensitive(&self, exp: &str) -> bool {
        if self.input.len() < self.cursor + exp.len() {
            return false;
        }

        let cursor = self.cursor;
        let exp: Vec<char> = exp.to_lowercase().chars().collect();
        for i in 0..exp.len() {
            if exp[i] != self.input[cursor + i].to_ascii_lowercase() {
                return false;
            }
        }
        true
    }

    /// If there is a `needle` after the `self.cursor` position, that position is returned.
    pub fn find(&mut self, needle: char) -> Option<usize> {
        let bgn = self.cursor;
        if self.input.len() <= bgn {
            return None;
        }
        for i in bgn..self.input.len() {
            if self.input[i] == needle {
                return Some(i);
            }
        }
        return None;
    }

    /// If there is a `needle` after the `self.cursor` position, that position is returned.
    pub fn find_str(&mut self, needle: &str) -> Option<usize> {
        let needle: Vec<char> = needle.chars().collect();
        let mut i = self.cursor;
        if self.input.len() <= i {
            return None;
        }

        let mut bgn_idx;
        while i < self.input.len() {
            // first character
            if self.input[i] == needle[0] {
                if needle.len() == 1 {
                    return Some(i);
                }
                bgn_idx = i;
                i += 1;
            } else {
                i += 1;
                continue;
            }

            // second and subsequent characters
            let mut j = 1;
            while j < needle.len() {
                if self.input.len() <= i {
                    return None;
                }

                if self.input[i] == needle[j] {
                    if j == needle.len() - 1 {
                        return Some(bgn_idx);
                    }
                } else {
                    break;
                }
                i += 1;
                j += 1;
            }

            i = bgn_idx + 1;
        }

        None
    }

    /// Returns the character at the `cursor` position.
    #[allow(dead_code)]
    pub fn get_char(&self, cursor: usize) -> Result<char, String> {
        if self.input.len() <= cursor {
            return Err(String::from("out of input"));
        }
        Ok(self.input[cursor])
    }

    /// Returns from `bgn` to `end` as a String.
    pub fn get_string(&self, bgn: usize, end: usize) -> Result<String, String> {
        if end <= bgn {
            return Err(String::from("invalid range"));
        }

        if self.input.len() <= end {
            return Err(format!("out of input {:?}:{:?}", bgn, end));
        }

        let mut s = String::new();
        for i in bgn..end {
            s.push(self.input[i]);
        }

        Ok(s)
    }
}
