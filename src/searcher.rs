use crate::dom::tag;
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
/// let mut needle = Tag::new("li".to_string());
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
        if tag::satisfy_sufficient_condition(needle, tag) {
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
        if tag::satisfy_sufficient_condition(needle, tag) {
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
/// use parsercher::dom::Dom;
/// use parsercher::dom::DomType;
/// use parsercher::dom::Tag;
///
/// let doc = r#"
/// <body>
///   <ul id="list1" class="targetList">
///     <li class="key1">1-1</li>
///     <li class="key2"><span>1-2</span></li>
///   </ul>
///
///   <ul id="list2">
///     <li class="key1">2-1</li>
///     <li>2-2</li>
///   </ul>
///
///   <div>
///     <div>
///       <ul id="list3" class="targetList">
///         <li class="key1">3-1</li>
///         <li class="item">3-2</li>
///         <li class="key2">3-3</li>
///       </ul>
///     </div>
///   </div>
///
///   <ul id="list4">
///     <li class="key1">4-1</li>
///     <li class="key2">4-2</li>
///   </ul>
/// </body>
/// "#;
///
/// // <ul class="targetList">
/// let mut ul_dom = Dom::new(DomType::Tag);
/// let mut ul_tag = Tag::new("ul".to_string());
/// ul_tag.set_attr("class", "targetList");
/// ul_dom.set_tag(ul_tag);
///
/// // <li class="key1">
/// let mut li_dom1 = Dom::new(DomType::Tag);
/// let mut li_tag = Tag::new("li".to_string());
/// li_tag.set_attr("class", "key1");
/// li_dom1.set_tag(li_tag);
///
/// // <li class="key2">
/// let mut li_dom2 = Dom::new(DomType::Tag);
/// let mut li_tag = Tag::new("li".to_string());
/// li_tag.set_attr("class", "key2");
/// li_dom2.set_tag(li_tag);
///
/// // <ul class="targetList">
/// //   <li class="key1"></li>
/// //   <li class="key2"></li>
/// // </ul>
/// ul_dom.add_child(li_dom1);
/// ul_dom.add_child(li_dom2);
///
/// if let Ok(root_dom) = parsercher::parse(&doc) {
///     if let Some(res) = parsercher::search_dom(&root_dom, &ul_dom) {
///         parsercher::print_dom_tree(&res);
///     }
/// }
/// ```
/// output:
/// ```text
/// <root>
///   <ul class="targetList" id="list1">
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
    search_dom_exe(&mut res, dom, needle, false);
    match res.get_children() {
        Some(_) => return Some(res),
        None => return None,
    }
}

/// # Arguments
///
/// * `parent_match` - True if the parents of `needle` match.
fn search_dom_exe(res: &mut Dom, dom: &Dom, needle: &Dom, parent_match: bool) -> bool {
    // Comparison of Dom structures
    let mut dom_match = false;
    if dom.dom_type == needle.dom_type {
        match dom.dom_type {
            DomType::Tag => {
                if let Some(tag) = dom.get_tag() {
                    if let Some(needle_tag) = needle.get_tag() {
                        dom_match = tag::satisfy_sufficient_condition(needle_tag, tag);
                    }
                }
            }
            DomType::Text => {
                if let Some(text) = dom.get_text() {
                    if let Some(needle_text) = needle.get_text() {
                        if text.get_text() == needle_text.get_text() {
                            dom_match = true;
                        }
                    }
                }
            }
            DomType::Comment => {
                if let Some(comment) = dom.get_comment() {
                    if let Some(needle_comment) = needle.get_comment() {
                        if comment.get_comment() == needle_comment.get_comment() {
                            dom_match = true;
                        }
                    }
                }
            }
        }
    }

    if dom_match {
        // If the parent of `dom` do not match and there are no children of` needle`,
        // then `needle` is only leaves, so clone all the remaining trees.
        if let None = needle.get_children() {
            if !parent_match {
                res.add_child(dom.clone());
                return true;
            }
        }

        if let Some(dom_children) = dom.get_children() {
            if let Some(needle_children) = needle.get_children() {
                if dom_children.len() < needle_children.len() {
                    return false;
                }
            }
        }

        // Count the number of dom children that match the `needle` children.
        let mut child_match_cnt = 0;
        if let Some(dom_children) = dom.get_children() {
            if let Some(needle_children) = needle.get_children() {
                for dom_child in dom_children.iter() {
                    for needle_child in needle_children.iter() {
                        if search_dom_exe(res, dom_child, needle_child, dom_match) {
                            child_match_cnt += 1;
                        }
                    }
                }
            }
        }

        // Returns false if dom doesn't have all the `needle` children.
        if let Some(needle_children) = needle.get_children() {
            if child_match_cnt < needle_children.len() {
                return false;
            }
        }

        // If the parents of `dom` do not match, then` dom` is the root,
        // so clone all the remaining trees.
        if !parent_match {
            res.add_child(dom.clone());
            return true;
        }

        if dom_match {
            return true;
        }
    } else {
        if let Some(children) = dom.get_children() {
            for child in children.iter() {
                search_dom_exe(res, child, needle, dom_match);
            }
        }
    }

    false
}
