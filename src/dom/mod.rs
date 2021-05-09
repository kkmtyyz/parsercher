//! Module for representing a tree of Dom structures.

pub mod comment;
pub mod tag;
pub mod text;

pub use comment::Comment;
pub use tag::Tag;
pub use text::Text;

/// Type of Dom structure.
#[derive(Debug, PartialEq)]
pub enum DomType {
    Tag,
    Text,
    Comment,
}

/// A structure that represents the parsing result of a tag document.
#[derive(Debug)]
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
}
