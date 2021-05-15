use crate::dom::tag::Tag;
use crate::dom::Dom;
use crate::dom::DomType;

/// Returns Tag structures from which the needle is a sufficient condition from the Dom structure tree.
///
/// # Examples
/// Get `li` tags that `class` attribute value is `target` from the following HTML.
/// ```text
/// <ol>
///    <li class="target">first</li>
///    <li>second</li>
///    <li id="third" class="target">therd</li>
/// </ol>
/// ```
///
/// ```compile_fail
/// let mut needle = Tag::new("li");
/// needle.set_attr("class", "target");
/// if let Some(tags) = parsercher::search_tag(&dom, &needle) {
///     println!("{:#?}", tags);
/// }
/// ```
/// output:
/// ```text
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
        if Tag::p_implies_q(needle, tag) {
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
/// ```text
/// <body>
///    <h1 class="h1">section1</h1>
///    <h2 class="h2">section2</h2>
///    <h3 class="h3">section3</h3>
/// </body>
/// ```
///
/// ```compile_fail
/// if let Some(tags) = parsercher::search_tag_from_name(&dom, "h2") {
///     println!("{:#?}", tags);
/// }
/// ```
///
/// output:
/// ```text
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
/// ```text
/// <ol>
///    <li class="target">first</li>
///    <li>second</li>
///    <li class="target">therd</li>
/// </ol>
/// ```
///
/// ```compile_fail
/// let mut needle = Tag::new("li");
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
        if Tag::p_implies_q(needle, tag) {
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

/// Returns partial trees from the Dom structure tree.
/// Duplicate everything below the subtree that matches the `needle` tree.
///
/// The Tag structures contained in the `needle` tree are evaluated under sufficient conditions.
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
/// let needle_dom = parsercher::parse(&needle).unwrap();
/// // Remove `root`dom of needle_dom
/// let needle_dom = needle_dom.get_children().unwrap().get(0).unwrap();
///
/// if let Some(dom) = parsercher::search_dom(&root_dom, &needle_dom) {
///     parsercher::print_dom_tree(&dom);
/// }
/// ```
/// output:
/// ```text
/// <root>
///   <ul id="list1" class="targetList">
///     <li class="key1">
///       TEXT: "1-1"
///     <li class="key2">
///       <span>
///         TEXT: "1-2"
///   <ul class="targetList" id="list3">
///     <li class="key1">
///       TEXT: "3-1"
///     <li class="item">
///       TEXT: "3-2"
///     <li class="key2">
///       TEXT: "3-3"
/// ```
pub fn search_dom(dom: &Dom, needle: &Dom) -> Option<Dom> {
    let mut res = Dom::new_root();
    search_dom_exe(&mut res, dom, needle);
    match res.get_children() {
        Some(_) => return Some(res),
        None => return None,
    }
}

fn search_dom_exe(res: &mut Dom, dom: &Dom, needle: &Dom) {
    if Dom::p_implies_q(needle, dom) {
        if Dom::p_implies_q_tree(needle, dom) {
            res.add_child(dom.clone());
            return;
        }
    }
    match dom.get_children() {
        Some(children) => {
            for child in children.iter() {
                search_dom_exe(res, child, needle);
            }
        }
        None => return,
    }
}

/// Returns the value of a specific attribute for all tags.
///
/// # Examples
/// Get the value of the `target` attribute of all tags.
///
/// ```rust
/// use parsercher;
///
/// let html = r#"
/// <!DOCTYPE html>
/// <html>
///   <head>
///     <meta charset="UTF-8">
///     <meta target="value1">
///     <title>sample html</title>
///   </head>
///   <body target="value2">
///     <h1>sample</h1>
///
///     <div id="content" target="value3"></div>
///
///     <ol>
///       <li>first</li>
///       <li target="value4">second</li>
///       <li>therd</li>
///     </ol>
///   </body>
/// </html>
/// "#;
///
/// let dom = parsercher::parse(&html).unwrap();
///
/// let values = parsercher::search_attr(&dom, "target").unwrap();
/// assert_eq!(values.len(), 4);
/// assert_eq!(values[0], "value1".to_string());
/// assert_eq!(values[1], "value2".to_string());
/// assert_eq!(values[2], "value3".to_string());
/// assert_eq!(values[3], "value4".to_string());
/// ```
pub fn search_attr(dom: &Dom, attr: &str) -> Option<Vec<String>> {
    let mut res: Vec<String> = Vec::new();
    search_attr_exe(&mut res, dom, attr);
    if res.is_empty() {
        return None;
    }
    Some(res)
}

fn search_attr_exe(res: &mut Vec<String>, dom: &Dom, attr: &str) {
    if DomType::Tag == dom.dom_type {
        if let Some(tag) = dom.get_tag() {
            if let Some(value) = tag.get_attr(attr) {
                res.push(value);
            }
        }
    }

    if let Some(children) = dom.get_children() {
        for child in children.iter() {
            search_attr_exe(res, child, attr);
        }
    }
}
