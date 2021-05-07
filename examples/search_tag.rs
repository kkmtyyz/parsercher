use parsercher;
use parsercher::dom::Tag;
use std::collections::HashMap;

fn main() {
    let doc = r#"
<ol>
   <li class="target">first</li>
   <li>second</li>
   <li id="third" class="target">therd</li>
</ol>
"#;

    if let Ok(dom) = parsercher::parse(&doc) {
        let mut needle = Tag::new("li".to_string());
        let mut attr = HashMap::new();
        attr.insert("class".to_string(), "target".to_string());
        needle.set_attr(attr);
        if let Some(tags) = parsercher::search_tag(&dom, &needle) {
            println!("{:#?}", tags);
        }
    }
}
