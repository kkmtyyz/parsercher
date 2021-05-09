# parsercher

[![Crate](https://img.shields.io/crates/v/parsercher.svg)](https://crates.io/crates/parsercher)
[![API](https://img.shields.io/badge/api-2.0.0-green.svg)](https://docs.rs/parsercher)

**Parses and searches Tag documents. (e.g. HTML, XML)**

parsercher parses documents written in tags such as HTML and XML.
- Create a tree of Dom structures from the tag document.
- Search for tags and text in the tree of Dom structures.

## Usage
Add this to your `Cargo.toml`:
```
[dependencies]
parsercher = "2.0.0"
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)
## Examples
**Example of getting text from HTML.**  
Create a tree of Dom structure from HTML and get the text of `li` tag that value of `class` attribute is `target`.
```rust
use parsercher;
use parsercher::dom::Tag;

let html = r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <title>sample html</title>
  </head>
  <body>
    <ol>
      <li class="target">first</li>
      <li>second</li>
      <li class="target">therd</li>
    </ol>
  </body>
</html>
"#;

if let Ok(root_dom) = parsercher::parse(&html) {
    let mut needle = Tag::new("li".to_string());
    needle.set_attr("class", "target");

    if let Some(texts) = parsercher::search_text_from_tag_children(&root_dom, &needle) {
        assert_eq!(texts.len(), 2);
        assert_eq!(texts[0], "first".to_string());
        assert_eq!(texts[1], "therd".to_string());
    }
}
```

**More complex examples of Dom structure tree**
```rust
use parsercher;

let html = r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <title>sample html</title>
  </head>
  <body>
    <h1>Hello, world!</h1>

    <div id="content"></div>

    <ol>
      <li>first</li>
      <li>second</li>
      <li>therd</li>
    </ol>
    <!-- All script code becomes one text -->
<script>
  let content = document.getElementById('content');
  content.textContent = 'content';
</script>
  </body>
</html>
"#;

if let Ok(dom) = parsercher::parse(&html) {
    println!("{:#?}", dom);
}
```

output:
```
Dom {
    dom_type: Tag,
    tag: Some(
        Tag {
            name: "root",
            attr: None,
            terminated: false,
            terminator: false,
        },
    ),
    text: None,
    comment: None,
    children: Some(
        [
            Dom {
                dom_type: Tag,
                tag: Some(
                    Tag {
                        name: "!DOCTYPE",
                        attr: Some(
                            {
                                "html": "",
                            },
                        ),
                        terminated: false,
                        terminator: false,
                    },
                ),
                text: None,
                comment: None,
                children: None,
            },
            Dom {
                dom_type: Tag,
                tag: Some(
                    Tag {
                        name: "html",
                        attr: None,
                        terminated: false,
                        terminator: false,
                    },
                ),
                text: None,
                comment: None,
                children: Some(
                    [
                        Dom {
                            dom_type: Tag,
                            tag: Some(
                                Tag {
                                    name: "head",
                                    attr: None,
                                    terminated: false,
                                    terminator: false,
                                },
                            ),
                            text: None,
                            comment: None,
                            children: Some(
                                [
                                    Dom {
                                        dom_type: Tag,
                                        tag: Some(
                                            Tag {
                                                name: "meta",
                                                attr: Some(
                                                    {
                                                        "charset": "UTF-8",
                                                    },
                                                ),
                                                terminated: false,
                                                terminator: false,
                                            },
                                        ),
                                        text: None,
                                        comment: None,
                                        children: None,
                                    },
                                    Dom {
                                        dom_type: Tag,
                                        tag: Some(
                                            Tag {
                                                name: "title",
                                                attr: None,
                                                terminated: false,
                                                terminator: false,
                                            },
                                        ),
                                        text: None,
                                        comment: None,
                                        children: Some(
                                            [
                                                Dom {
                                                    dom_type: Text,
                                                    tag: None,
                                                    text: Some(
                                                        Text {
                                                            text: "sample html",
                                                        },
                                                    ),
                                                    comment: None,
                                                    children: None,
                                                },
                                            ],
                                        ),
                                    },
                                ],
                            ),
                        },
                        Dom {
                            dom_type: Tag,
                            tag: Some(
                                Tag {
                                    name: "body",
                                    attr: None,
                                    terminated: false,
                                    terminator: false,
                                },
                            ),
                            text: None,
                            comment: None,
                            children: Some(
                                [
                                    Dom {
                                        dom_type: Tag,
                                        tag: Some(
                                            Tag {
                                                name: "h1",
                                                attr: None,
                                                terminated: false,
                                                terminator: false,
                                            },
                                        ),
                                        text: None,
                                        comment: None,
                                        children: Some(
                                            [
                                                Dom {
                                                    dom_type: Text,
                                                    tag: None,
                                                    text: Some(
                                                        Text {
                                                            text: "Hello, world!",
                                                        },
                                                    ),
                                                    comment: None,
                                                    children: None,
                                                },
                                            ],
                                        ),
                                    },
                                    Dom {
                                        dom_type: Tag,
                                        tag: Some(
                                            Tag {
                                                name: "div",
                                                attr: Some(
                                                    {
                                                        "id": "content",
                                                    },
                                                ),
                                                terminated: false,
                                                terminator: false,
                                            },
                                        ),
                                        text: None,
                                        comment: None,
                                        children: None,
                                    },
                                    Dom {
                                        dom_type: Tag,
                                        tag: Some(
                                            Tag {
                                                name: "ol",
                                                attr: None,
                                                terminated: false,
                                                terminator: false,
                                            },
                                        ),
                                        text: None,
                                        comment: None,
                                        children: Some(
                                            [
                                                Dom {
                                                    dom_type: Tag,
                                                    tag: Some(
                                                        Tag {
                                                            name: "li",
                                                            attr: None,
                                                            terminated: false,
                                                            terminator: false,
                                                        },
                                                    ),
                                                    text: None,
                                                    comment: None,
                                                    children: Some(
                                                        [
                                                            Dom {
                                                                dom_type: Text,
                                                                tag: None,
                                                                text: Some(
                                                                    Text {
                                                                        text: "first",
                                                                    },
                                                                ),
                                                                comment: None,
                                                                children: None,
                                                            },
                                                        ],
                                                    ),
                                                },
                                                Dom {
                                                    dom_type: Tag,
                                                    tag: Some(
                                                        Tag {
                                                            name: "li",
                                                            attr: None,
                                                            terminated: false,
                                                            terminator: false,
                                                        },
                                                    ),
                                                    text: None,
                                                    comment: None,
                                                    children: Some(
                                                        [
                                                            Dom {
                                                                dom_type: Text,
                                                                tag: None,
                                                                text: Some(
                                                                    Text {
                                                                        text: "second",
                                                                    },
                                                                ),
                                                                comment: None,
                                                                children: None,
                                                            },
                                                        ],
                                                    ),
                                                },
                                                Dom {
                                                    dom_type: Tag,
                                                    tag: Some(
                                                        Tag {
                                                            name: "li",
                                                            attr: None,
                                                            terminated: false,
                                                            terminator: false,
                                                        },
                                                    ),
                                                    text: None,
                                                    comment: None,
                                                    children: Some(
                                                        [
                                                            Dom {
                                                                dom_type: Text,
                                                                tag: None,
                                                                text: Some(
                                                                    Text {
                                                                        text: "therd",
                                                                    },
                                                                ),
                                                                comment: None,
                                                                children: None,
                                                            },
                                                        ],
                                                    ),
                                                },
                                            ],
                                        ),
                                    },
                                    Dom {
                                        dom_type: Comment,
                                        tag: None,
                                        text: None,
                                        comment: Some(
                                            Comment {
                                                comment: " All script code becomes one text ",
                                            },
                                        ),
                                        children: None,
                                    },
                                    Dom {
                                        dom_type: Tag,
                                        tag: Some(
                                            Tag {
                                                name: "script",
                                                attr: None,
                                                terminated: false,
                                                terminator: false,
                                            },
                                        ),
                                        text: None,
                                        comment: None,
                                        children: Some(
                                            [
                                                Dom {
                                                    dom_type: Text,
                                                    tag: None,
                                                    text: Some(
                                                        Text {
                                                            text: "\n  let content = document.getElementById(\'content\');\n  content.textContent = \'content\';\n",
                                                        },
                                                    ),
                                                    comment: None,
                                                    children: None,
                                                },
                                            ],
                                        ),
                                    },
                                ],
                            ),
                        },
                    ],
                ),
            },
        ],
    ),
}
```
