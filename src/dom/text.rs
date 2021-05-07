//! Module of Text structure.

/// A structure that expresses something other than tags and comments.
#[derive(Debug, Clone)]
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
