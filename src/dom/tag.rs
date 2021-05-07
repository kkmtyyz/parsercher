//! Module of Tag structure.

use std::collections::HashMap;

/// A structure that represents a tag element.
/// grammar: `<[/]name [attr[="value"]] [/]>`
#[derive(Debug, Clone)]
pub struct Tag {
    name: String,
    attr: Option<HashMap<String, String>>,
    terminated: bool,
    terminator: bool,
}

impl Tag {
    /// Create new Tag structer
    ///
    /// # Arguments
    /// * `name` - If `<h1 class="section1">`, then `h1`.
    pub fn new(name: String) -> Tag {
        Tag {
            name,
            attr: None,
            terminated: false,
            terminator: false,
        }
    }

    /// Returns the tag name.
    /// If `<li class="item">`, then `li`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Set attributes.
    ///
    /// # Arguments
    /// * `attr` - Attribute name is key. Attribute value is value.
    ///
    /// # Examples
    /// For `<h1 id="title" class="section1">`:
    /// ```rust,no_run
    /// let mut tag = Tag::new("h1".to_string());
    /// let mut attr: HashMap<String, String> = HashMap::new();
    /// attr.insert("id".to_string(), "title".to_string());
    /// attr.insert("class".to_string(), "section1".to_string());
    /// tag.set_attr(attr);
    /// ```
    pub fn set_attr(&mut self, attr: HashMap<String, String>) {
        self.attr = Some(attr);
    }

    /// Returns attributes.
    /// If there is no attribute, `None` is returned.
    ///
    /// # Examples
    /// For `<h1 id="title" class="section1">`:
    /// ```rust,no_run
    /// if let Some(attr) = tag.get_attr() {
    ///     assert_eq!(attr.get("id".to_string()), "title".to_string());
    ///     assert_eq!(attr.get("class".to_string()), "section1".to_string());
    /// }
    /// ```
    pub fn get_attr(&self) -> Option<&HashMap<String, String>> {
        self.attr.as_ref()
    }

    /// Set true to represent tags that are self-closed.
    ///
    /// # Examples
    /// `<area/>`
    pub fn set_terminated(&mut self, b: bool) {
        self.terminated = b;
    }

    /// Returns true if the self-closed tag.
    ///
    /// # Examples
    /// `<area/>`
    pub fn is_terminated(&self) -> bool {
        self.terminated
    }

    /// Set true to indicate that there is a terminator tag.
    ///
    /// # Examples
    /// `<h1></h1>`
    pub fn set_terminator(&mut self, b: bool) {
        self.terminator = b;
    }

    /// Returns true if there is a terminator tag.
    ///
    /// # Examples
    /// `<h1></h1>`
    pub fn is_terminator(&self) -> bool {
        self.terminator
    }
}
