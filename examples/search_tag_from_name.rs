use parsercher;

fn main() {
    let doc = r#"
<body>
   <h1 class="h1">section1</h1>
   <h2 class="h2">section2</h2>
   <h3 class="h3">section3</h3>
</body>
"#;

    if let Ok(dom) = parsercher::parse(&doc) {
        if let Some(tags) = parsercher::search_tag_from_name(&dom, "h2") {
            println!("{:#?}", tags);
        }
    }
}
