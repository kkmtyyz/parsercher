use parsercher;

fn main() {
    let html = r#"
    <!DOCTYPE html>
    <html>
      <head>
        <meta charset="UTF-8">
        <meta target="value1">
        <title>sample html</title>
      </head>
      <body target="value2">
        <h1>sample</h1>

        <div id="content" target="value3"></div>

        <ol>
          <li>first</li>
          <li target="value4">second</li>
          <li>therd</li>
        </ol>
      </body>
    </html>
    "#;

    let dom = parsercher::parse(&html).unwrap();

    let values = parsercher::search_attr(&dom, "target").unwrap();
    assert_eq!(values.len(), 4);
    assert_eq!(values[0], "value1".to_string());
    assert_eq!(values[1], "value2".to_string());
    assert_eq!(values[2], "value3".to_string());
    assert_eq!(values[3], "value4".to_string());
}
