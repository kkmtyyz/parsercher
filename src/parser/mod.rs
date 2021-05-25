mod input;

use std::collections::HashMap;

use crate::dom::comment::Comment;
use crate::dom::tag::Tag;
use crate::dom::text::Text;
use crate::dom::Dom;
use crate::dom::DomType;
use input::Input;

/// Parses the tag document and returns a Dom structure tree.
///
/// # Arguments
/// * `doc` - tag document
///
/// # Errors
/// * If the document ends in the middle of a tag or double quote.
///
/// # Examples
/// ```rust
/// let html = r#"
/// <body>
///   <h1 class="h1">Hello</h1>
/// </body>
/// }"#;
///
/// if let Ok(dom) = parsercher::parse(&html) {
///     println!("{:#?}", dom);
/// }
/// ```
///
/// output:
///
/// ```text
/// Dom {
///     dom_type: Tag,
///     tag: Some(
///         Tag {
///             name: "root",
///             attr: None,
///             terminated: false,
///             terminator: false,
///         },
///     ),
///     text: None,
///     comment: None,
///     children: Some(
///         [
///             Dom {
///                 dom_type: Tag,
///                 tag: Some(
///                     Tag {
///                         name: "body",
///                         attr: None,
///                         terminated: false,
///                         terminator: false,
///                     },
///                 ),
///                 text: None,
///                 comment: None,
///                 children: Some(
///                     [
///                         Dom {
///                             dom_type: Tag,
///                             tag: Some(
///                                 Tag {
///                                     name: "h1",
///                                     attr: Some(
///                                         {
///                                             "class": "h1",
///                                         },
///                                     ),
///                                     terminated: false,
///                                     terminator: false,
///                                 },
///                             ),
///                             text: None,
///                             comment: None,
///                             children: Some(
///                                 [
///                                     Dom {
///                                         dom_type: Text,
///                                         tag: None,
///                                         text: Some(
///                                             Text {
///                                                 text: "Hello",
///                                             },
///                                         ),
///                                         comment: None,
///                                         children: None,
///                                     },
///                                 ],
///                             ),
///                         },
///                     ],
///                 ),
///             },
///         ],
///     ),
/// }
/// ```
///
pub fn parse(doc: &str) -> Result<Dom, String> {
    let mut input = Input::new(doc);
    let mut dom_vec = create_dom_vec(&mut input)?;
    //debug_print_dom_vec(&dom_vec);

    let mut root_dom = Dom::new_root();
    create_dom_tree(&mut dom_vec, &mut root_dom);
    Ok(root_dom)
}

/// Returns the value of the tag's attribute.
///
/// State to receive:
/// The cursor points to the first '"' or first '\''.
/// "<value>"
/// or
/// '<value>'
fn parse_tag_attr_value(input: &mut Input, dlmt: char) -> Result<String, String> {
    input.next(); // move cursor to after '"' or '\''
    let value_bgn = input.get_cursor();

    let value_end;
    match input.find(dlmt) {
        Some(cursor) => value_end = cursor,
        None => return Err(String::from("Input ends in the middle of double quote")),
    }

    if value_bgn == value_end {
        // value is empty
        return Ok(String::new());
    }

    input.set_cursor(value_end);
    input.get_string(value_bgn, value_end)
}

/// Parse tag attributes.
///
/// State to receive:
/// The cursor points to the first character of <attr>.
/// <attr>[ = "<value>"] [/]>
/// or
/// <attr>[ = '<value>'] [/]>
fn parse_tag_attr(input: &mut Input, mut tag: Tag) -> Result<Tag, String> {
    // get the end position of the tag
    let tag_end;
    match input.find('>') {
        Some(cursor) => tag_end = cursor,
        None => return Err(String::from("Input ends in the middle of the tag")),
    }

    let mut attr_map = HashMap::new();

    // get attributes and their value
    // The terminal '/' is also an attribute
    loop {
        if input.expect('>') {
            input.next();
            break;
        }

        let attr_bgn = input.get_cursor();
        let mut attr_end = tag_end;

        // If the tag contains '=', that position is the end position of the attribute name
        if let Some(cursor) = input.find('=') {
            if cursor < tag_end {
                attr_end = cursor;
            }
        }

        // If the tag contains an ' ' and it precedes '=',
        // make that position the end position of the attribute name.
        if let Some(cursor) = input.find(' ') {
            if cursor < attr_end {
                attr_end = cursor;
            }
        }

        input.set_cursor(attr_end);
        let attr_name = input.get_string(attr_bgn, attr_end)?;

        // get value
        let mut value = String::new();
        if input.get_cursor() != tag_end {
            // If the tag contains an '='
            if let Some(cursor) = input.find('=') {
                if cursor < tag_end {
                    input.set_cursor(cursor); // move cursor to '='
                    input.next_char(); // move cursor to after '='
                    if input.expect('"') {
                        match parse_tag_attr_value(input, '"') {
                            Ok(v) => value = v,
                            Err(e) => return Err(e),
                        }
                    } else if input.expect('\'') {
                        match parse_tag_attr_value(input, '\'') {
                            Ok(v) => value = v,
                            Err(e) => return Err(e),
                        }
                    }
                }
            }
        }

        attr_map.insert(attr_name, value);

        if input.expect('>') {
            input.next();
            break;
        }

        input.next_char();
    }

    // If the attribute contains '/', remove it
    if let Some(_) = attr_map.remove("/") {
        tag.set_terminated(true);
    }

    tag.set_attrs(attr_map);

    Ok(tag)
}

/// Parse the tag name.
///
/// State to receive:
/// The cursor points to the first character of <tag_name>.
/// <tag_name> [<attr>[="<value>"]] [/]>
/// or
/// <tag_name> [<attr>[='<value>']] [/]>
fn parse_tag_name(input: &mut Input, terminator: bool) -> Result<Tag, String> {
    // Get the start position of the tag name
    let name_bgn = input.get_cursor();

    // get the end position of the tag
    let tag_end;
    match input.find('>') {
        Some(cursor) => tag_end = cursor,
        None => return Err(String::from("Input ends in the middle of the tag")),
    }

    let mut name_end = tag_end;

    // If the tag contains ' ', make that position the end position of the tag name.
    if let Some(cursor) = input.find(' ') {
        if cursor < tag_end {
            name_end = cursor;
        }
    }

    input.set_cursor(name_end);
    let mut tag = Tag::new(&input.get_string(name_bgn, name_end)?);
    tag.set_terminator(terminator);

    if input.expect('>') {
        input.next(); // move cursor to after '>'
        return Ok(tag);
    }

    input.next_char();

    // If there are spaces before the '>'
    if input.expect('>') {
        input.next();
        return Ok(tag);
    }

    return parse_tag_attr(input, tag);
}

/// Parses the tag and returns a Dom structure.
///
/// State to receive:
/// The cursor points to the first '<'.
/// <[/]<tag_name> [<attr>[="<value>"]] [/]>
/// or
/// <[/]<tag_name> [<attr>[='<value>']] [/]>
fn parse_tag(input: &mut Input) -> Result<Dom, String> {
    input.next(); // move cursor to after '<'

    let mut terminator = false;
    if input.expect('/') {
        input.next(); // move cursor to after '/'
        terminator = true;
    }

    let tag = parse_tag_name(input, terminator)?;
    // TODO debug
    //println!("{:#?}", tag);
    let mut dom = Dom::new(DomType::Tag);
    dom.set_tag(tag);
    return Ok(dom);
}

/// Parse comment.
///
/// State to receive:
/// The cursor points to the first '<'.
/// <!-- <comment> -->
fn parse_comment(input: &mut Input) -> Result<Dom, String> {
    // get the position after "<!--"
    let bgn = input.get_cursor() + "<!--".len();

    match input.find_str("-->") {
        Some(cursor) => {
            input.set_cursor(cursor + "-->".len()); // move cursor after "-->"
            let comment = Comment::new(&input.get_string(bgn, cursor)?);
            // TODO debug
            //println!("{:#?}", comment);
            let mut dom = Dom::new(DomType::Comment);
            dom.set_comment(comment);
            return Ok(dom);
        }
        None => return Err(String::from("Input ends in the middle of the comment")),
    }
}

/// Tet text.
fn parse_text(input: &mut Input) -> Result<Dom, String> {
    let bgn = input.get_cursor();

    let end;
    match input.find('<') {
        Some(cursor) => {
            input.set_cursor(cursor);
            end = cursor;
        }
        None => {
            input.next_char();
            end = input.get_cursor();
        }
    }

    let text = Text::new(&input.get_string(bgn, end)?);
    // TODO debug
    //println!("{:#?}", text);
    let mut dom = Dom::new(DomType::Text);
    dom.set_text(text);
    return Ok(dom);
}

/// Get the code of the script tag as text.
fn parse_text_script(input: &mut Input) -> Result<Dom, String> {
    let bgn = input.get_cursor();
    let end;

    match input.find_str("</script") {
        Some(cursor) => {
            input.set_cursor(cursor);
            end = cursor;
        }
        None => return Err(String::from("Input ends in the middle of the tag")),
    }

    let text = Text::new(&input.get_string(bgn, end)?);
    let mut dom = Dom::new(DomType::Text);
    dom.set_text(text);
    return Ok(dom);
}

/// Parse "<!doctype html>".
/// case insensitive.
///
/// State to receive:
/// The cursor points to the first '<'.
/// <!doctype html>
#[allow(dead_code)]
fn parse_doctype(input: &mut Input) -> Result<Dom, String> {
    if !input.expect_str_insensitive("<!doctype html>") {
        return Err(String::from("Input is not html"));
    }

    // Set the tag name to "doctype"
    input.next(); // move cursor to '!'
    input.next(); // move cursor to 'd'
    let bgn = input.get_cursor();
    let end = bgn + "doctype".len();
    input.set_cursor(end); // move cursor to the ' ' before the "html"
    let mut tag = Tag::new(&input.get_string(bgn, end)?);

    input.next(); // move cursor to 'h'

    // Set the attribute to "html"
    // The value of attribute is ""
    let mut attr: HashMap<String, String> = HashMap::new();
    let bgn = input.get_cursor();
    let end = bgn + "html".len();
    input.set_cursor(end); // move cursor to '>'
    attr.insert(input.get_string(bgn, end)?, String::new());
    tag.set_attrs(attr);

    let mut dom = Dom::new(DomType::Tag);
    dom.set_tag(tag);

    input.next(); // move cursor to after '>'

    Ok(dom)
}

/// Parses the tag document and returns the Vec of the Dom structure.
fn create_dom_vec(input: &mut Input) -> Result<Vec<Dom>, String> {
    let mut dom_vec: Vec<Dom> = Vec::new();

    // move cursor to the first '<'
    while !input.expect('<') {
        input.next_char();
    }

    /*
    // "<!doctype html>"
    match parse_doctype(input) {
        Ok(dom) => dom_vec.push(dom),
        Err(e) => return Err(e),
    }
    */

    while !input.is_end() {
        // TODO debug
        //println!("check: {}", input.get_char(input.get_cursor())?);
        if input.expect_str("<!--") {
            // comment
            match parse_comment(input) {
                Ok(dom) => dom_vec.push(dom),
                Err(e) => return Err(e),
            }
        } else if input.expect('<') {
            // tag
            match parse_tag(input) {
                Ok(dom) => {
                    // if the dom is script tag
                    let mut is_bgn_script = false;
                    if let DomType::Tag = dom.dom_type {
                        let tag = dom.get_tag().unwrap();
                        if tag.get_name() == "script" && !tag.is_terminator() {
                            is_bgn_script = true;
                        }
                    }

                    dom_vec.push(dom);

                    // if the dom is script tag and has text
                    if is_bgn_script && !input.expect('<') {
                        match parse_text_script(input) {
                            Ok(dom) => dom_vec.push(dom),
                            Err(e) => return Err(e),
                        }
                    }
                }
                Err(e) => return Err(e),
            }
        } else {
            if input.expect(' ') || input.expect('\n') {
                input.next_char(); // skip ' ' and '\n'
            }

            if !input.expect('<') {
                // text
                match parse_text(input) {
                    Ok(dom) => dom_vec.push(dom),
                    Err(e) => return Err(e),
                }
            }
        }
    }
    Ok(dom_vec)
}

/// dom_vec debugging function
#[allow(dead_code)]
fn debug_print_dom_vec(dom_vec: &Vec<Dom>) {
    for dom in dom_vec.iter() {
        match dom.dom_type {
            DomType::Tag => println!("{:#?}", dom.get_tag().unwrap()),
            DomType::Text => println!("{:#?}", dom.get_text().unwrap()),
            DomType::Comment => println!("{:#?}", dom.get_comment().unwrap()),
        }
    }
}

/// Find the end tag paired with `starter` from dom_vec and return its index.
fn search_terminator(dom_vec: &mut Vec<Dom>, starter: &Tag) -> Option<usize> {
    let mut i = 0;
    while i < dom_vec.len() {
        let dom = dom_vec.get(i).unwrap();
        if let DomType::Tag = dom.dom_type {
            let tag = dom.get_tag().unwrap();
            if tag.is_terminator() {
                if starter.get_name() == tag.get_name() {
                    return Some(i);
                }
            }
        }
        i += 1;
    }
    None
}

/// If the tag is not terminated, add it to the child, otherwise add it to the child.
fn create_dom_tree(dom_vec: &mut Vec<Dom>, parent: &mut Dom) {
    while !dom_vec.is_empty() {
        let mut dom = dom_vec.remove(0);

        if let DomType::Tag = dom.dom_type {
            let tag = dom.get_tag().unwrap();

            if tag.is_terminator() {
                // If tag is terminator. `</ tag>`
                return;
            }

            if !tag.is_terminated() {
                // If not self-terminating. not `<tag />`
                if let Some(terminator_idx) = search_terminator(dom_vec, tag) {
                    // If there is terminator tag
                    if terminator_idx == 0 {
                        // If there are no children, delete the terminator tag
                        dom_vec.remove(0);
                    } else {
                        // If there are children, recurse
                        create_dom_tree(dom_vec, &mut dom);
                    }
                }
            }
        }

        parent.add_child(dom);
    }
}

/// Output the Dom structure in a human readable format.
///
/// Using `println!`.
/// Useful for debugging.
///
/// # Examples
/// Output the following HTML.
/// ```rust
/// let html = r#"
/// <head>
///   <meta charset="UTF-8">
///   <title>sample</title>
/// </head>
/// <body>
///   <h1>Hello, world!</h1>
/// </body>
/// "#;
/// if let Ok(dom) = parsercher::parse(&html) {
///     parsercher::print_dom_tree(&dom);
/// }
/// ```
/// output:
/// ```text
/// <root>
///   <head>
///     <meta charset="UTF-8">
///     <title>
///       TEXT: "sample"
///   <body>
///     <h1>
///       TEXT: "Hello, world!"
/// ```
pub fn print_dom_tree(dom: &Dom) {
    print_dom_tree_exe(dom, 0);
}

fn print_dom_tree_exe(dom: &Dom, depth: usize) {
    for _ in 0..depth {
        print!("  ");
    }

    match dom.dom_type {
        DomType::Tag => {
            let tag = dom.get_tag().unwrap();
            print!("<{}", tag.get_name());
            if let Some(attrs) = tag.get_attrs() {
                for (attr, value) in attrs.iter() {
                    print!(" {}=\"{}\"", attr, value);
                }
            }
            println!(">");

            if let Some(children) = dom.get_children() {
                for child in children {
                    print_dom_tree_exe(child, depth + 1);
                }
            }
        }
        DomType::Text => {
            let text = dom.get_text().unwrap();
            let text = String::from(text.get_text());
            let text = text.replace("\n", "\\n");
            println!("TEXT: \"{}\"", text);
        }
        DomType::Comment => {
            let comment = dom.get_comment().unwrap();
            let comment = String::from(comment.get_comment());
            let comment = comment.replace("\n", "\\n");
            println!("<!--\"{}\"-->", comment);
        }
    }
}
