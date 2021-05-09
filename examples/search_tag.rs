use parsercher;
use parsercher::dom::Tag;

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
        needle.set_attr("class", "target");
        if let Some(tags) = parsercher::search_tag(&dom, &needle) {
            println!("{:#?}", tags);
        }
    }
}
