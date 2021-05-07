//! Module of Comment structure.

/// A structure that represents a comment tag.
/// grammar: `<!-- comment -->`
#[derive(Debug, Clone)]
pub struct Comment {
    comment: String,
}

impl Comment {
    /// Create new Comment structure.
    ///
    /// # Arguments
    /// * `comment` - If `<!-- hello -->`, then `hello`.
    pub fn new(comment: String) -> Comment {
        Comment { comment }
    }

    /// Returns the comment.
    /// If `<!-- hello -->`, then returns `hello`.
    pub fn get_comment(&self) -> &str {
        &self.comment
    }
}
