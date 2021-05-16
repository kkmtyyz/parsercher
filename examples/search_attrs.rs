use parsercher;

fn main() {
    let html = r#"
    <!DOCTYPE html>
    <html>
      <head>
        <meta charset="UTF-8">
        <meta id="id1">
        <title>sample html</title>
      </head>
      <body id="id2" class="class1">
        <h1>sample</h1>

        <div align="center" class="class2"></div>

        <ol>
          <li>first</li>
          <li class="class3">second</li>
          <li>therd</li>
        </ol>
      </body>
    </html>
    "#;

    let dom = parsercher::parse(&html).unwrap();

    let attrs = vec!["id", "class"];
    let values = parsercher::search_attrs(&dom, &attrs).unwrap();
    assert_eq!(values.len(), 5);
    assert_eq!(values[0], "id1".to_string());
    assert_eq!(values[1], "id2".to_string());
    assert_eq!(values[2], "class1".to_string());
    assert_eq!(values[3], "class2".to_string());
    assert_eq!(values[4], "class3".to_string());
}
