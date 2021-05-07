use crate::dom::tag::Tag;
use crate::dom::Dom;
use crate::dom::DomType;

/// Returns true if p is a sufficient condition for q.
/// `p => q`
///
/// # Examples
/// ```rust
/// let mut p = Tag::new("h1".to_string());
/// let mut attr = HashMap::new();
/// attr.insert("class".to_string(), "target".to_string());
/// p.set_attr(attr);
///
/// let mut q = Tag::new("h1".to_string());
/// let mut attr = HashMap::new();
/// attr.insert("id".to_string(), "q".to_string());
/// attr.insert("class".to_string(), "target".to_string());
/// q.set_attr(attr);
///
/// assert_eq!(parsercher::satisfy_sufficient_condition(&p, &q), true);
///
/// let mut q = Tag::new("h1".to_string());
/// let mut attr = HashMap::new();
/// attr.insert("id".to_string(), "q".to_string());
/// q.set_attr(attr);
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
        if let Some(p_attr) = p.get_attr() {
            match q.get_attr() {
                Some(q_attr) => {
                    for (p_key, p_value) in p_attr.iter() {
                        match q_attr.get(p_key) {
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

/// Returns Tag structures from which the needle is a sufficient condition from the Dom structure tree.
///
/// # Examples
/// Get `li` tags that `class` attribute value is `target` from the following HTML.
/// ```
/// <ol>
///    <li class="target">first</li>
///    <li>second</li>
///    <li id="third" class="target">therd</li>
/// </ol>
/// ```
///
/// ```rust,no_run
/// let mut needle = Tag::new("li".to_string());
/// let mut attr = HashMap::new();
/// attr.insert("class".to_string(), "target".to_string());
/// needle.set_attr(attr);
/// if let Some(tags) = parsercher::search_tag(&dom, &needle) {
///     println!("{:#?}", tags);
/// }
/// ```
/// output:
/// ```
/// [
///     Tag {
///         name: "li",
///         attr: Some(
///             {
///                 "class": "target",
///             },
///         ),
///         terminated: false,
///         terminator: false,
///     },
///     Tag {
///         name: "li",
///         attr: Some(
///             {
///                 "id": "third",
///                 "class": "target",
///             },
///         ),
///         terminated: false,
///         terminator: false,
///     },
/// ]
/// ```
pub fn search_tag(dom: &Dom, needle: &Tag) -> Option<Vec<Tag>> {
    let mut res: Vec<Tag> = Vec::new();
    search_tag_exe(&mut res, dom, needle);
    if res.is_empty() {
        return None;
    }
    Some(res)
}

fn search_tag_exe(res: &mut Vec<Tag>, dom: &Dom, needle: &Tag) {
    if let Some(tag) = dom.get_tag() {
        if satisfy_sufficient_condition(needle, tag) {
            res.push(tag.clone());
        }

        if let Some(children) = dom.get_children() {
            for child in children {
                search_tag_exe(res, child, needle);
            }
        }
    }
}

/// Returns Tag structures with a tag name equal to `name` from the Dom structure tree.
///
/// # Examples
/// Get only the `h2` tag from the following HTML.
/// ```
/// <body>
///    <h1 class="h1">section1</h1>
///    <h2 class="h2">section2</h2>
///    <h3 class="h3">section3</h3>
/// </body>
/// ```
///
/// ```rust,no_run
/// if let Some(tags) = parsercher::search_tag_from_name(&dom, "h2") {
///     println!("{:#?}", tags);
/// }
/// ```
///
/// output:
/// ```
/// [
///     Tag {
///         name: "h2",
///         attr: Some(
///             {
///                 "class": "h2",
///             },
///         ),
///         terminated: false,
///         terminator: false,
///     },
/// ]
/// ```
pub fn search_tag_from_name(dom: &Dom, name: &str) -> Option<Vec<Tag>> {
    let mut res: Vec<Tag> = Vec::new();
    search_tag_from_name_exe(&mut res, dom, name);
    if res.is_empty() {
        return None;
    }
    Some(res)
}

fn search_tag_from_name_exe(res: &mut Vec<Tag>, dom: &Dom, name: &str) {
    if let DomType::Tag = dom.dom_type {
        let tag = dom.get_tag().unwrap();
        if name == tag.get_name() {
            res.push(tag.clone());
        }

        if let Some(children) = dom.get_children() {
            for child in children {
                search_tag_from_name_exe(res, child, name);
            }
        }
    }
}

/// Returns texts of the child of the Tag structure for which `needle` is a sufficient condition from the Dom structure tree.
///
/// # Examples
/// Get just texts of `li` tags that `class` attribute value is `target` from the following HTML.
/// ```
/// <ol>
///    <li class="target">first</li>
///    <li>second</li>
///    <li class="target">therd</li>
/// </ol>
/// ```
///
/// ```
/// let mut needle = Tag::new("li".to_string());
/// let mut attr = HashMap::new();
/// attr.insert("class".to_string(), "target".to_string());
/// needle.set_attr(attr);
/// if let Some(texts) = parsercher::search_text_from_tag_children(&dom, &needle) {
///     assert_eq!(texts.len(), 2);
///     assert_eq!(texts[0], "first".to_string());
///     assert_eq!(texts[1], "therd".to_string());
/// }
/// ```
pub fn search_text_from_tag_children(dom: &Dom, needle: &Tag) -> Option<Vec<String>> {
    let mut res: Vec<String> = Vec::new();
    search_text_from_tag_children_exe(&mut res, dom, needle);
    if res.is_empty() {
        return None;
    }
    Some(res)
}

fn search_text_from_tag_children_exe(res: &mut Vec<String>, dom: &Dom, needle: &Tag) {
    if let Some(tag) = dom.get_tag() {
        if satisfy_sufficient_condition(needle, tag) {
            if let Some(children) = dom.get_children() {
                for child in children {
                    if let Some(text) = child.get_text() {
                        res.push(text.get_text().to_string());
                    }
                }
            }
        }

        if let Some(children) = dom.get_children() {
            for child in children {
                search_text_from_tag_children_exe(res, child, needle);
            }
        }
    }
}
