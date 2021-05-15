//! Module of Tag structure.

use std::collections::HashMap;

/// A structure that represents a tag element.
/// grammar: `<[/]name [attr[="value"]] [/]>`
#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
    name: String,
    attrs: Option<HashMap<String, String>>,
    terminated: bool,
    terminator: bool,
}

impl Tag {
    /// Create new Tag structer
    ///
    /// # Arguments
    /// * `name` - If `<h1 class="section1">`, then `h1`.
    pub fn new(name: &str) -> Tag {
        Tag {
            name: String::from(name),
            attrs: None,
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
    /// ```rust
    /// use std::collections::HashMap;
    /// use parsercher::dom::Tag;
    ///
    /// let mut tag = Tag::new("h1");
    /// let mut attrs: HashMap<String, String> = HashMap::new();
    /// attrs.insert("id".to_string(), "title".to_string());
    /// attrs.insert("class".to_string(), "section1".to_string());
    /// tag.set_attrs(attrs);
    /// ```
    pub fn set_attrs(&mut self, attrs: HashMap<String, String>) {
        self.attrs = Some(attrs);
    }

    /// Returns attributes.
    /// If there is no attribute, `None` is returned.
    ///
    /// # Examples
    /// For `<h1 id="title" class="section1">`:
    /// ```
    /// use parsercher::dom::Tag;
    ///
    /// let mut tag = Tag::new("h1");
    /// tag.set_attr("id", "title");
    /// tag.set_attr("class", "section1");
    ///
    /// if let Some(attrs) = tag.get_attrs() {
    ///     assert_eq!(attrs.get(&"id".to_string()), Some(&"title".to_string()));
    ///     assert_eq!(attrs.get(&"class".to_string()), Some(&"section1".to_string()));
    /// }
    /// ```
    pub fn get_attrs(&self) -> Option<&HashMap<String, String>> {
        self.attrs.as_ref()
    }

    /// Set attribute.
    ///
    /// # Examples
    /// For `<h1 id="title" class="section1">`:
    /// ```
    /// use parsercher::dom::Tag;
    ///
    /// let mut tag = Tag::new("h1");
    /// tag.set_attr("id", "title");
    /// tag.set_attr("class", "section1");
    /// ```
    pub fn set_attr(&mut self, attr: &str, value: &str) {
        match self.attrs.as_mut() {
            Some(attrs) => {
                attrs.insert(String::from(attr), String::from(value));
            }
            None => {
                let mut attrs = HashMap::new();
                attrs.insert(String::from(attr), String::from(value));
                self.attrs = Some(attrs);
            }
        };
    }

    /// Returns the value of the specified attribute.
    ///
    /// # Examples
    /// For `<h1 id="title" class="section1">`:
    /// ```
    /// use parsercher::dom::Tag;
    ///
    /// let mut tag = Tag::new("h1");
    /// tag.set_attr("id", "title");
    /// tag.set_attr("class", "section1");
    ///
    /// if let Some(value) = tag.get_attr("class") {
    ///     assert_eq!(value, "section1".to_string());
    /// }
    /// ```
    pub fn get_attr(&self, attr: &str) -> Option<String> {
        if let Some(attrs) = &self.attrs {
            if let Some(v) = attrs.get(attr) {
                return Some(v.to_string());
            }
        }
        None
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

/// Returns true if p is a sufficient condition for q.
/// `p => q`
///
/// # Examples
/// ```rust
/// use std::collections::HashMap;
/// use parsercher::dom::Tag;
///
/// let mut p = Tag::new("h1");
/// p.set_attr("class", "target");
///
/// let mut q = Tag::new("h1");
/// q.set_attr("id", "q");
/// q.set_attr("class", "target");
///
/// assert_eq!(parsercher::satisfy_sufficient_condition(&p, &q), true);
///
/// let mut q = Tag::new("h1");
/// q.set_attr("id", "q");
///
/// assert_eq!(parsercher::satisfy_sufficient_condition(&p, &q), false);
/// ```
pub fn satisfy_sufficient_condition(p: &Tag, q: &Tag) -> bool {
    let mut satisfied = false;
    // name
    if q.get_name() == p.get_name() {
        satisfied = true;
    }

    // attr
    if satisfied {
        if let Some(p_attrs) = p.get_attrs() {
            match q.get_attrs() {
                Some(q_attrs) => {
                    for (p_key, p_value) in p_attrs.iter() {
                        match q_attrs.get(p_key) {
                            Some(q_value) => {
                                if p_value != "" && p_value != q_value {
                                    satisfied = false;
                                    break;
                                }
                            }
                            None => {
                                satisfied = false;
                                break;
                            }
                        }
                    }
                }
                None => satisfied = false,
            }
        }
    }
    satisfied
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_empty_attr() {
        let tag = Tag::new("h1");
        assert_eq!(None, tag.get_attr("class"));
    }

    #[test]
    fn set_attr() {
        let mut tag = Tag::new("h1");
        tag.set_attr("class", "section1");
        assert_eq!(Some("section1".to_string()), tag.get_attr("class"));
        assert_eq!(None, tag.get_attr("id"));
    }

    #[test]
    fn sufficient_condition() {
        let mut p = Tag::new("h1");
        p.set_attr("class", "target");

        let mut q = Tag::new("h1");
        q.set_attr("id", "q");
        q.set_attr("class", "target");

        assert_eq!(satisfy_sufficient_condition(&p, &q), true);
    }

    #[test]
    fn not_sufficient_condition() {
        let mut p = Tag::new("h1");
        p.set_attr("class", "target");

        let mut q = Tag::new("h1");
        q.set_attr("id", "q");

        assert_eq!(satisfy_sufficient_condition(&p, &q), false);
    }

    #[test]
    fn eq_test() {
        let mut a = Tag::new("h1");
        a.set_attr("id", "idA");
        a.set_attr("class", "classA");

        let mut b = Tag::new("h1");
        b.set_attr("id", "idA");
        b.set_attr("class", "classA");

        assert_eq!(a == b, true);
        assert_eq!(a != b, false);
    }

    #[test]
    fn ne_test() {
        let mut a = Tag::new("h1");
        a.set_attr("id", "idA");
        a.set_attr("class", "classA");

        let mut b = Tag::new("h2");
        b.set_attr("id", "idB");
        b.set_attr("class", "classB");

        assert_eq!(a == b, false);
        assert_eq!(a != b, true);
    }
}
