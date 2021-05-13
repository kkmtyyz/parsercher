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

    <ul id="list1" class="targetList">
      <li class="key1">1-1</li>
      <li class="key2"><span>1-2</span></li>
    </ul>

    <ul id="list2">
      <li class="key1">2-1</li>
      <li>2-2</li>
    </ul>

    <div>
      <div>
        <ul class="targetList">
          <ul id="list3" class="targetList">
            <li class="key1">3-1</li>
            <li class="item">3-2</li>
            <li class="key2">3-3</li>
          </ul>
        </ul>
      </div>
    </div>

    <ul id="list4">
      <li class="key1">4-1</li>
      <li class="key2">4-2</li>
    </ul>

  </body>
</html>
"#;

    let root_dom = parsercher::parse(&html).unwrap();

    let needle = r#"
<ul class="targetList">
  <li class="key1"></li>
  <li class="key2"></li>
</ul>
"#;
    let needle_dom = parsercher::parse(&needle).unwrap();
    // Remove `root`dom of needle_dom
    let needle_dom = needle_dom.get_children().unwrap().get(0).unwrap();

    if let Some(dom) = parsercher::search_dom(&root_dom, &needle_dom) {
        parsercher::print_dom_tree(&dom);
    }
}
