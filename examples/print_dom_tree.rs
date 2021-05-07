use parsercher;

fn main() {
    let html = r#"
<head>
    <meta charset="UTF-8">
    <title>sample</title>
</head>
<body>
    <h1>Hello, world!</h1>
</body>
"#;
    if let Ok(dom) = parsercher::parse(&html) {
        parsercher::print_dom_tree(&dom);
    }
}
