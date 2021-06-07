use parsercher;

fn main() {
    let html = r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <title>sample html</title>
  </head>
  <body>
    <h1>Hello, world!</h1>

    <div 
    id="content"></div>

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
}
