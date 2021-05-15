use parsercher;
use parsercher::dom::Tag;

fn main() {
    let doc = r#"
<ol>
   <li class="target">first</li>
   <li>second</li>
   <li class="target">therd</li>
</ol>
"#;

    if let Ok(dom) = parsercher::parse(&doc) {
        let mut needle = Tag::new("li");
        needle.set_attr("class", "target");
        if let Some(texts) = parsercher::search_text_from_tag_children(&dom, &needle) {
            assert_eq!(texts.len(), 2);
            assert_eq!(texts[0], "first".to_string());
            assert_eq!(texts[1], "therd".to_string());
        }
    }
}
