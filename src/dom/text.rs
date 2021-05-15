//! Module of Text structure.

/// A structure that expresses something other than tags and comments.
#[derive(Debug, PartialEq, Clone)]
pub struct Text {
    text: String,
}

impl Text {
    /// Create new Text structure.
    ///
    /// # Arguments
    /// * `text` - If `<h1>section</h1>`, then `section`.
    pub fn new(text: String) -> Text {
        Text { text }
    }

    /// Returns the text.
    /// If `<h1>section</h1>`, then returns `section`.
    pub fn get_text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_test() {
        let a = Text::new("a text".to_string());
        let b = Text::new("a text".to_string());
        assert_eq!(a == b, true);
        assert_eq!(a != b, false);
    }

    #[test]
    fn ne_test() {
        let a = Text::new("a text".to_string());
        let b = Text::new("b text".to_string());
        assert_eq!(a != b, true);
        assert_eq!(a == b, false);
    }
}
