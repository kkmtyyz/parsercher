//!# Parses and searches Tag documents. (e.g. HTML, XML)
//!
//! parsercher parses documents written in tags such as HTML and XML.
//! * Create a Dom structure tree from the tag document.
//! * Search for tags and text from the Dom structure tree.
//! * Search subtrees from the Dom structure tree.
//!
//! # Usage
//! Add this to your `Cargo.toml`:
//! ```text
//! [dependencies]
//! parsercher = "3.1.3"
//! ```
//!
//! # Examples
//! **Example of getting text from HTML.**  
//! Create a tree of Dom structure from HTML and get the text of `li` tag that value of `class` attribute is `target`.
//! ```rust
//! use parsercher;
//! use parsercher::dom::Tag;
//!
//! let html = r#"
//! <!DOCTYPE html>
//! <html>
//!   <head>
//!     <meta charset="UTF-8">
//!     <title>sample html</title>
//!   </head>
//!   <body>
//!     <ol>
//!       <li class="target">first</li>
//!       <li>second</li>
//!       <li class="target">therd</li>
//!     </ol>
//!   </body>
//! </html>
//! "#;
//!
//! if let Ok(root_dom) = parsercher::parse(&html) {
//!     let mut needle = Tag::new("li");
//!     needle.set_attr("class", "target");
//!
//!     if let Some(texts) = parsercher::search_text_from_tag_children(&root_dom, &needle) {
//!         assert_eq!(texts.len(), 2);
//!         assert_eq!(texts[0], "first".to_string());
//!         assert_eq!(texts[1], "therd".to_string());
//!     }
//! }
//! ```
//!
//! **Example of searching a subtree from the Dom structure tree.**
//!
//! Find a subtree that has a `ul` tag whose value in the `class` attribute is `targetList` and
//! two `li` tags under it. Also, the values of the `class` attribute of the `li` tag must be
//! `key1` and` key2`, respectively.
//!
//! Looking for:
//! ```text
//! <ul class="targetList">
//!   <li class="key1"></li>
//!   <li class="key2"></li>
//! </ul>
//! ```
//!
//! ```rust
//! use parsercher;
//!
//! let doc = r#"
//!   <body>
//!     <ul id="list1" class="targetList">
//!       <li class="key1">1-1</li>
//!       <li class="key2">
//!         <span>1-2</span>
//!       </li>
//!     </ul>
//!
//!     <ul id="list2">
//!       <li class="key1">2-1</li>
//!       <li>2-2</li>
//!     </ul>
//!
//!     <div>
//!       <div>
//!         <ul class="targetList">
//!           <ul id="list3" class="targetList">
//!             <li class="key1">3-1</li>
//!             <li class="item">3-2</li>
//!             <li class="key2">3-3</li>
//!           </ul>
//!         </ul>
//!       </div>
//!     </div>
//!
//!     <ul id="list4">
//!       <li class="key1">4-1</li>
//!       <li class="key2">4-2</li>
//!     </ul>
//!   </body>
//! "#;
//!
//! let root_dom = parsercher::parse(&doc).unwrap();
//!
//! let needle = r#"
//! <ul class="targetList">
//!   <li class="key1"></li>
//!   <li class="key2"></li>
//! </ul>
//! "#;
//! let needle_dom = parsercher::parse(&needle).unwrap();
//! // Remove `root`dom of needle_dom
//! let needle_dom = needle_dom.get_children().unwrap().get(0).unwrap();
//!
//! if let Some(dom) = parsercher::search_dom(&root_dom, &needle_dom) {
//!     parsercher::print_dom_tree(&dom);
//! }
//! ```
//! output:
//! ```text
//! <root>
//!   <ul id="list1" class="targetList">
//!     <li class="key1">
//!       TEXT: "1-1"
//!     <li class="key2">
//!       <span>
//!         TEXT: "1-2"
//!   <ul id="list3" class="targetList">
//!     <li class="key1">
//!       TEXT: "3-1"
//!     <li class="item">
//!       TEXT: "3-2"
//!     <li class="key2">
//!       TEXT: "3-3"
//! ```
//!
//! **More complex examples of Dom structure tree**
//! ```rust
//! use parsercher;
//!
//! let html = r#"
//! <!DOCTYPE html>
//! <html>
//!   <head>
//!     <meta charset="UTF-8">
//!     <title>sample html</title>
//!   </head>
//!   <body>
//!     <h1>Hello, world!</h1>
//!
//!     <div id="content"></div>
//!
//!     <ol>
//!       <li>first</li>
//!       <li>second</li>
//!       <li>therd</li>
//!     </ol>
//!     <!-- All script code becomes one text -->
//! <script>
//!   let content = document.getElementById('content');
//!   content.textContent = 'content';
//! </script>
//!   </body>
//! </html>
//! "#;
//!
//! if let Ok(dom) = parsercher::parse(&html) {
//!     println!("{:#?}", dom);
//! }
//! ```
//!
//! output:
//! ```text
//! Dom {
//!     dom_type: Tag,
//!     tag: Some(
//!         Tag {
//!             name: "root",
//!             attr: None,
//!             terminated: false,
//!             terminator: false,
//!         },
//!     ),
//!     text: None,
//!     comment: None,
//!     children: Some(
//!         [
//!             Dom {
//!                 dom_type: Tag,
//!                 tag: Some(
//!                     Tag {
//!                         name: "!DOCTYPE",
//!                         attr: Some(
//!                             {
//!                                 "html": "",
//!                             },
//!                         ),
//!                         terminated: false,
//!                         terminator: false,
//!                     },
//!                 ),
//!                 text: None,
//!                 comment: None,
//!                 children: None,
//!             },
//!             Dom {
//!                 dom_type: Tag,
//!                 tag: Some(
//!                     Tag {
//!                         name: "html",
//!                         attr: None,
//!                         terminated: false,
//!                         terminator: false,
//!                     },
//!                 ),
//!                 text: None,
//!                 comment: None,
//!                 children: Some(
//!                     [
//!                         Dom {
//!                             dom_type: Tag,
//!                             tag: Some(
//!                                 Tag {
//!                                     name: "head",
//!                                     attr: None,
//!                                     terminated: false,
//!                                     terminator: false,
//!                                 },
//!                             ),
//!                             text: None,
//!                             comment: None,
//!                             children: Some(
//!                                 [
//!                                     Dom {
//!                                         dom_type: Tag,
//!                                         tag: Some(
//!                                             Tag {
//!                                                 name: "meta",
//!                                                 attr: Some(
//!                                                     {
//!                                                         "charset": "UTF-8",
//!                                                     },
//!                                                 ),
//!                                                 terminated: false,
//!                                                 terminator: false,
//!                                             },
//!                                         ),
//!                                         text: None,
//!                                         comment: None,
//!                                         children: None,
//!                                     },
//!                                     Dom {
//!                                         dom_type: Tag,
//!                                         tag: Some(
//!                                             Tag {
//!                                                 name: "title",
//!                                                 attr: None,
//!                                                 terminated: false,
//!                                                 terminator: false,
//!                                             },
//!                                         ),
//!                                         text: None,
//!                                         comment: None,
//!                                         children: Some(
//!                                             [
//!                                                 Dom {
//!                                                     dom_type: Text,
//!                                                     tag: None,
//!                                                     text: Some(
//!                                                         Text {
//!                                                             text: "sample html",
//!                                                         },
//!                                                     ),
//!                                                     comment: None,
//!                                                     children: None,
//!                                                 },
//!                                             ],
//!                                         ),
//!                                     },
//!                                 ],
//!                             ),
//!                         },
//!                         Dom {
//!                             dom_type: Tag,
//!                             tag: Some(
//!                                 Tag {
//!                                     name: "body",
//!                                     attr: None,
//!                                     terminated: false,
//!                                     terminator: false,
//!                                 },
//!                             ),
//!                             text: None,
//!                             comment: None,
//!                             children: Some(
//!                                 [
//!                                     Dom {
//!                                         dom_type: Tag,
//!                                         tag: Some(
//!                                             Tag {
//!                                                 name: "h1",
//!                                                 attr: None,
//!                                                 terminated: false,
//!                                                 terminator: false,
//!                                             },
//!                                         ),
//!                                         text: None,
//!                                         comment: None,
//!                                         children: Some(
//!                                             [
//!                                                 Dom {
//!                                                     dom_type: Text,
//!                                                     tag: None,
//!                                                     text: Some(
//!                                                         Text {
//!                                                             text: "Hello, world!",
//!                                                         },
//!                                                     ),
//!                                                     comment: None,
//!                                                     children: None,
//!                                                 },
//!                                             ],
//!                                         ),
//!                                     },
//!                                     Dom {
//!                                         dom_type: Tag,
//!                                         tag: Some(
//!                                             Tag {
//!                                                 name: "div",
//!                                                 attr: Some(
//!                                                     {
//!                                                         "id": "content",
//!                                                     },
//!                                                 ),
//!                                                 terminated: false,
//!                                                 terminator: false,
//!                                             },
//!                                         ),
//!                                         text: None,
//!                                         comment: None,
//!                                         children: None,
//!                                     },
//!                                     Dom {
//!                                         dom_type: Tag,
//!                                         tag: Some(
//!                                             Tag {
//!                                                 name: "ol",
//!                                                 attr: None,
//!                                                 terminated: false,
//!                                                 terminator: false,
//!                                             },
//!                                         ),
//!                                         text: None,
//!                                         comment: None,
//!                                         children: Some(
//!                                             [
//!                                                 Dom {
//!                                                     dom_type: Tag,
//!                                                     tag: Some(
//!                                                         Tag {
//!                                                             name: "li",
//!                                                             attr: None,
//!                                                             terminated: false,
//!                                                             terminator: false,
//!                                                         },
//!                                                     ),
//!                                                     text: None,
//!                                                     comment: None,
//!                                                     children: Some(
//!                                                         [
//!                                                             Dom {
//!                                                                 dom_type: Text,
//!                                                                 tag: None,
//!                                                                 text: Some(
//!                                                                     Text {
//!                                                                         text: "first",
//!                                                                     },
//!                                                                 ),
//!                                                                 comment: None,
//!                                                                 children: None,
//!                                                             },
//!                                                         ],
//!                                                     ),
//!                                                 },
//!                                                 Dom {
//!                                                     dom_type: Tag,
//!                                                     tag: Some(
//!                                                         Tag {
//!                                                             name: "li",
//!                                                             attr: None,
//!                                                             terminated: false,
//!                                                             terminator: false,
//!                                                         },
//!                                                     ),
//!                                                     text: None,
//!                                                     comment: None,
//!                                                     children: Some(
//!                                                         [
//!                                                             Dom {
//!                                                                 dom_type: Text,
//!                                                                 tag: None,
//!                                                                 text: Some(
//!                                                                     Text {
//!                                                                         text: "second",
//!                                                                     },
//!                                                                 ),
//!                                                                 comment: None,
//!                                                                 children: None,
//!                                                             },
//!                                                         ],
//!                                                     ),
//!                                                 },
//!                                                 Dom {
//!                                                     dom_type: Tag,
//!                                                     tag: Some(
//!                                                         Tag {
//!                                                             name: "li",
//!                                                             attr: None,
//!                                                             terminated: false,
//!                                                             terminator: false,
//!                                                         },
//!                                                     ),
//!                                                     text: None,
//!                                                     comment: None,
//!                                                     children: Some(
//!                                                         [
//!                                                             Dom {
//!                                                                 dom_type: Text,
//!                                                                 tag: None,
//!                                                                 text: Some(
//!                                                                     Text {
//!                                                                         text: "therd",
//!                                                                     },
//!                                                                 ),
//!                                                                 comment: None,
//!                                                                 children: None,
//!                                                             },
//!                                                         ],
//!                                                     ),
//!                                                 },
//!                                             ],
//!                                         ),
//!                                     },
//!                                     Dom {
//!                                         dom_type: Comment,
//!                                         tag: None,
//!                                         text: None,
//!                                         comment: Some(
//!                                             Comment {
//!                                                 comment: " All script code becomes one text ",
//!                                             },
//!                                         ),
//!                                         children: None,
//!                                     },
//!                                     Dom {
//!                                         dom_type: Tag,
//!                                         tag: Some(
//!                                             Tag {
//!                                                 name: "script",
//!                                                 attr: None,
//!                                                 terminated: false,
//!                                                 terminator: false,
//!                                             },
//!                                         ),
//!                                         text: None,
//!                                         comment: None,
//!                                         children: Some(
//!                                             [
//!                                                 Dom {
//!                                                     dom_type: Text,
//!                                                     tag: None,
//!                                                     text: Some(
//!                                                         Text {
//!                                                             text: "\n  let content = document.getElementById(\'content\');\n  content.textContent = \'content\';\n",
//!                                                         },
//!                                                     ),
//!                                                     comment: None,
//!                                                     children: None,
//!                                                 },
//!                                             ],
//!                                         ),
//!                                     },
//!                                 ],
//!                             ),
//!                         },
//!                     ],
//!                 ),
//!             },
//!         ],
//!     ),
//! }
//! ```
//!

pub mod dom;
mod parser;
mod searcher;

pub use parser::parse;
pub use parser::print_dom_tree;

pub use searcher::search_attr;
pub use searcher::search_attrs;
pub use searcher::search_dom;
pub use searcher::search_tag;
pub use searcher::search_tag_from_name;
pub use searcher::search_text_from_tag_children;
