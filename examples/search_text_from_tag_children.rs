use parsercher;
use parsercher::dom::Tag;
use std::collections::HashMap;

fn main() {
    let doc = r#"
<ol>
   <li class="target">first</li>
   <li>second</li>
   <li class="target">therd</li>
</ol>
"#;

    if let Ok(dom) = parsercher::parse(&doc) {
        let mut needle = Tag::new("li".to_string());
        let mut attr = HashMap::new();
        attr.insert("class".to_string(), "target".to_string());
        needle.set_attr(attr);
        if let Some(texts) = parsercher::search_text_from_tag_children(&dom, &needle) {
            assert_eq!(texts.len(), 2);
            assert_eq!(texts[0], "first".to_string());
            assert_eq!(texts[1], "therd".to_string());
        }
    }
}
