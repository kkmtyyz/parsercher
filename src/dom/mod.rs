//! Module for representing a tree of Dom structures.

pub mod comment;
pub mod tag;
pub mod text;

pub use comment::Comment;
pub use tag::Tag;
pub use text::Text;

use crate::parser;
use crate::searcher;

/// Type of Dom structure.
#[derive(Debug, PartialEq, Clone)]
pub enum DomType {
    Tag,
    Text,
    Comment,
}

/// A structure that represents the parsing result of a tag document.
#[derive(Debug, PartialEq, Clone)]
pub struct Dom {
    /// Type of Dom structure
    pub dom_type: DomType,
    tag: Option<Tag>,
    text: Option<Text>,
    comment: Option<Comment>,
    children: Option<Vec<Box<Dom>>>,
}

impl Dom {
    /// Create new Dom structure.
    pub fn new(dom_type: DomType) -> Dom {
        Dom {
            dom_type,
            tag: None,
            text: None,
            comment: None,
            children: None,
        }
    }

    /// Create the new root dom.
    ///
    /// The root dom has a Tag structure whose name is root.
    pub fn new_root() -> Dom {
        let tag = Tag::new("root");
        let mut dom = Dom::new(DomType::Tag);
        dom.set_tag(tag);
        dom
    }

    fn domtype_str(&self) -> String {
        match self.dom_type {
            DomType::Tag => return String::from("Tag"),
            DomType::Text => return String::from("Text"),
            DomType::Comment => return String::from("Comment"),
        }
    }

    /// Set Tag structure.
    ///
    /// # Panics
    /// `self.dom_type` is not `DomType::Tag`
    pub fn set_tag(&mut self, tag: Tag) {
        match self.dom_type {
            DomType::Tag => self.tag = Some(tag),
            _ => panic!("invalid DomType. expect Tag but {}", self.domtype_str()),
        }
    }

    /// Returns the Tag structure.
    /// If it does not have a Tag structure, it returns `None`.
    pub fn get_tag(&self) -> Option<&Tag> {
        self.tag.as_ref()
    }

    /// Set Text structure.
    ///
    /// # Panics
    /// `self.dom_type` is not `DomType::Text`
    pub fn set_text(&mut self, text: Text) {
        match self.dom_type {
            DomType::Text => self.text = Some(text),
            _ => panic!("invalid DomType. expect Text but {}", self.domtype_str()),
        }
    }

    /// Returns the Text structure.
    /// If it does not have a Text structure, it returns `None`.
    pub fn get_text(&self) -> Option<&Text> {
        self.text.as_ref()
    }

    /// Set Comment structure.
    ///
    /// # Panics
    /// `self.dom_type` is not `DomType::Comment`
    pub fn set_comment(&mut self, comment: Comment) {
        match self.dom_type {
            DomType::Comment => self.comment = Some(comment),
            _ => panic!("invalid DomType. expect Comment but {}", self.domtype_str()),
        }
    }

    /// Returns the Comment structure.
    /// If it does not have a Comment structure, it returns `None`.
    pub fn get_comment(&self) -> Option<&Comment> {
        self.comment.as_ref()
    }

    /// Add a child Dom structure.
    pub fn add_child(&mut self, dom: Dom) {
        let dom = Box::new(dom);
        match &mut self.children {
            Some(children) => {
                children.push(dom);
            }
            None => {
                let mut children = Vec::new();
                children.push(dom);
                self.children = Some(children);
            }
        }
    }

    /// Returns child Dom structures as Vec.
    /// If it does not have children, it returns `None`.
    pub fn get_children(&self) -> Option<&Vec<Box<Dom>>> {
        self.children.as_ref()
    }

    /// Returns true if p is a sufficient condition for q.
    /// `p => q`
    ///
    /// # Examples
    /// ```rust
    /// use parsercher::dom::DomType;
    /// use parsercher::dom::Dom;
    /// use parsercher::dom::Tag;
    ///
    /// let mut p = Dom::new(DomType::Tag);
    /// let mut tag = Tag::new("h1");
    /// tag.set_attr("class", "target");
    /// p.set_tag(tag);
    ///
    /// let mut q = Dom::new(DomType::Tag);
    /// let mut tag = Tag::new("h1");
    /// tag.set_attr("id", "q");
    /// tag.set_attr("class", "target");
    /// q.set_tag(tag);
    ///
    /// assert_eq!(Dom::p_implies_q(&p, &q), true);
    ///
    /// let mut q = Dom::new(DomType::Tag);
    /// let mut tag = Tag::new("h1");
    /// tag.set_attr("id", "q");
    /// q.set_tag(tag);
    ///
    /// assert_eq!(Dom::p_implies_q(&p, &q), false);
    /// ```
    pub fn p_implies_q(p: &Dom, q: &Dom) -> bool {
        if q.dom_type != p.dom_type {
            return false;
        }
        match q.dom_type {
            DomType::Tag => {
                if let Some(q_tag) = q.get_tag() {
                    if let Some(p_tag) = p.get_tag() {
                        return Tag::p_implies_q(p_tag, q_tag);
                    }
                }
            }
            DomType::Text => {
                if let Some(q_text) = q.get_text() {
                    if let Some(p_text) = p.get_text() {
                        if q_text.get_text().contains(p_text.get_text()) {
                            return true;
                        }
                    }
                }
            }
            DomType::Comment => {
                if let Some(q_comment) = q.get_comment() {
                    if let Some(p_comment) = p.get_comment() {
                        if q_comment.get_comment().contains(p_comment.get_comment()) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Returns true if p is a sufficient condition for q.
    /// Compare the entire tree. `p => q`
    ///
    /// # Examples
    /// ```rust
    /// use parsercher;
    /// use parsercher::dom::Dom;
    ///
    /// // p
    /// let p = r#"
    /// <h1>
    ///   <div></div>
    ///   <ul>
    ///     <li></li>
    ///   </ul>
    /// </h1>
    /// "#;
    ///
    /// let p_dom = parsercher::parse(&p).unwrap();
    /// // Remove `root`dom of p_dom
    /// let p_dom = p_dom.get_children().unwrap().get(0).unwrap();
    ///
    /// // q
    /// let q = r#"
    /// <h1>
    ///   <div id="divId"></div>
    ///   <ul>
    ///     <li></li>
    ///   </ul>
    ///   <span></span>
    /// </h1>
    /// "#;
    ///
    /// let q_dom = parsercher::parse(&q).unwrap();
    /// // Remove `root`dom of p_dom
    /// let q_dom = q_dom.get_children().unwrap().get(0).unwrap();
    ///
    /// assert_eq!(Dom::p_implies_q_tree(&p_dom, &q_dom), true);
    /// ```
    pub fn p_implies_q_tree(p: &Dom, q: &Dom) -> bool {
        if !Dom::p_implies_q(p, q) {
            return false;
        }
        if let None = p.get_children() {
            return true;
        }
        if let None = q.get_children() {
            return false;
        }

        let p_children = p.get_children().unwrap();
        let q_children = q.get_children().unwrap();
        for p_child in p_children.iter() {
            let mut child_match = false;
            for q_child in q_children.iter() {
                if Dom::p_implies_q_tree(p_child, q_child) {
                    child_match = true;
                    break;
                }
            }
            // If the same child as p is not in the child of q.
            if !child_match {
                return false;
            }
        }
        true
    }

    /// Return the `needle`-like subtree from the Dom structure tree.
    /// The `needle` argument must be parsable html.
    ///
    /// # Examples
    /// Get the subtree that satisfies the following tag names and attribute values.
    /// ```text
    /// <ul class="targetList">
    ///   <li class="key1></li>
    ///   <li class="key2></li>
    /// </ul>
    /// ```
    ///
    /// ```rust
    /// use parsercher;
    ///
    /// let html = r#"
    /// <body>
    ///   <ul id="list1" class="targetList">
    ///     <li class="key1">1-1</li>
    ///     <li class="key2">
    ///       <span>1-2</span>
    ///     </li>
    ///   </ul>
    ///
    ///   <ul id="list2">
    ///     <li class="key1">2-1</li>
    ///     <li>2-2</li>
    ///   </ul>
    ///
    ///   <div>
    ///     <div>
    ///       <ul class="targetList">
    ///         <ul id="list3" class="targetList">
    ///           <li class="key1">3-1</li>
    ///           <li class="item">3-2</li>
    ///           <li class="key2">3-3</li>
    ///         </ul>
    ///       </ul>
    ///     </div>
    ///   </div>
    ///
    ///   <ul id="list4">
    ///     <li class="key1">4-1</li>
    ///     <li class="key2">4-2</li>
    ///   </ul>
    ///
    /// </body>
    /// "#;
    ///
    /// let root_dom = parsercher::parse(&html).unwrap();
    ///
    /// let needle = r#"
    /// <ul class="targetList">
    ///   <li class="key1"></li>
    ///   <li class="key2"></li>
    /// </ul>
    /// "#;
    /// let result = root_dom.search(&needle).unwrap().unwrap();
    ///
    /// assert!(result.len() == 2);
    ///
    /// for dom in result.iter() {
    ///     parsercher::print_dom_tree(&dom);
    /// }
    ///
    /// ```
    /// output:
    /// ```text
    /// <ul class="targetList" id="list1">
    ///   <li class="key1">
    ///     TEXT: "1-1"
    ///   <li class="key2">
    ///     <span>
    ///       TEXT: "1-2"
    /// <ul class="targetList" id="list3">
    ///   <li class="key1">
    ///     TEXT: "3-1"
    ///   <li class="item">
    ///     TEXT: "3-2"
    ///   <li class="key2">
    ///     TEXT: "3-3"
    /// ```
    pub fn search(&self, needle: &str) -> Result<Option<Vec<Box<Dom>>>, String> {
        let needle = parser::parse(&needle)?;
        // remove root dom
        let needle = needle.get_children().unwrap().get(0).unwrap();
        let root_dom = match searcher::search_dom(&self, &needle) {
            Some(root_dom) => root_dom,
            None => return Ok(None),
        };
        match root_dom.get_children() {
            Some(children) => Ok(Some(children.clone())),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn sufficient_condition() {
        let mut p = Dom::new(DomType::Tag);
        let mut tag = Tag::new("h1");
        tag.set_attr("class", "target");
        p.set_tag(tag);

        let mut q = Dom::new(DomType::Tag);
        let mut tag = Tag::new("h1");
        tag.set_attr("id", "q");
        tag.set_attr("class", "target");
        q.set_tag(tag);

        assert_eq!(Dom::p_implies_q(&p, &q), true);
    }

    #[test]
    fn not_sufficient_condition() {
        let mut p = Dom::new(DomType::Tag);
        let mut tag = Tag::new("h1");
        tag.set_attr("class", "target");
        p.set_tag(tag);

        let mut q = Dom::new(DomType::Tag);
        let mut tag = Tag::new("h1");
        tag.set_attr("id", "q");
        q.set_tag(tag);

        assert_eq!(Dom::p_implies_q(&p, &q), false);
    }

    #[test]
    fn text_sufficient_condition() {
        let mut p = Dom::new(DomType::Text);
        let text = Text::new("def");
        p.set_text(text);

        let mut q = Dom::new(DomType::Text);
        let text = Text::new("abcdefghi");
        q.set_text(text);

        assert_eq!(Dom::p_implies_q(&p, &q), true);
    }

    #[test]
    fn p_implies_q_tree_test() {
        // <h1>
        //   <div>
        //   <ul>
        //     <li>
        let mut p = Dom::new(DomType::Tag);
        let h1_tag = Tag::new("h1");
        p.set_tag(h1_tag);
        // div
        let mut div_dom = Dom::new(DomType::Tag);
        let div_tag = Tag::new("div");
        div_dom.set_tag(div_tag);
        p.add_child(div_dom);
        // ul
        let mut ul_dom = Dom::new(DomType::Tag);
        let ul_tag = Tag::new("ul");
        ul_dom.set_tag(ul_tag);
        // li
        let mut li_dom = Dom::new(DomType::Tag);
        let li_tag = Tag::new("li");
        li_dom.set_tag(li_tag);
        ul_dom.add_child(li_dom);
        p.add_child(ul_dom);

        // <h1>
        //   <div id="divid">
        //   <ul>
        //     <li>
        let mut q = Dom::new(DomType::Tag);
        let h1_tag = Tag::new("h1");
        q.set_tag(h1_tag);
        // div
        let mut div_dom = Dom::new(DomType::Tag);
        let mut div_tag = Tag::new("div");
        div_tag.set_attr("id", "divid");
        div_dom.set_tag(div_tag);
        q.add_child(div_dom);
        // ul
        let mut ul_dom = Dom::new(DomType::Tag);
        let ul_tag = Tag::new("ul");
        ul_dom.set_tag(ul_tag);
        // li
        let mut li_dom = Dom::new(DomType::Tag);
        let li_tag = Tag::new("li");
        li_dom.set_tag(li_tag);
        ul_dom.add_child(li_dom);
        q.add_child(ul_dom);

        assert_eq!(Dom::p_implies_q_tree(&p, &q), true);
    }

    #[test]
    fn eq_test() {
        let a = r#"
        <head>
          <title>sample</title>
        </head>
        <body>
          <h1>section</h1>
          <ul>
            <li>list1</li>
            <li>list2</li>
          </ul>
        </body>
        "#;
        let a_dom = parser::parse(&a).unwrap();

        let b = r#"
        <head>
          <title>sample</title>
        </head>
        <body>
          <h1>section</h1>
          <ul>
            <li>list1</li>
            <li>list2</li>
          </ul>
        </body>
        "#;
        let b_dom = parser::parse(&b).unwrap();

        assert_eq!(a_dom == b_dom, true);
        assert_eq!(a_dom != b_dom, false);
    }

    #[test]
    fn ne_test() {
        let a = r#"
        <head>
          <title>sample</title>
        </head>
        <body>
          <h1>section</h1>
          <ul>
            <li>list1</li>
            <li>list2</li>
          </ul>
        </body>
        "#;
        let a_dom = parser::parse(&a).unwrap();

        let b = r#"
        <head>
          <title>sample</title>
        </head>
        <body>
          <h1>section</h1>
          <ul>
            <li>list1</li>
            <li>list3</li>
          </ul>
        </body>
        "#;
        let b_dom = parser::parse(&b).unwrap();

        assert_eq!(a_dom == b_dom, false);
        assert_eq!(a_dom != b_dom, true);
    }
}
