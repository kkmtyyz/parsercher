//! Module of Comment structure.

/// A structure that represents a comment tag.
/// grammar: `<!-- comment -->`
#[derive(Debug, PartialEq, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_test() {
        let a = Comment::new("a comment".to_string());
        let b = Comment::new("a comment".to_string());
        assert_eq!(a == b, true);
        assert_eq!(a != b, false);
    }

    #[test]
    fn ne_test() {
        let a = Comment::new("a comment".to_string());
        let b = Comment::new("b comment".to_string());
        assert_eq!(a != b, true);
        assert_eq!(a == b, false);
    }
}
