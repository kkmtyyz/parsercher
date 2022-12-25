extern crate parsercher;

use parsercher::dom::DomType;

#[test]
fn search_dom() {
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

    let dom = parsercher::parse(&html).unwrap();
    //parsercher::print_dom_tree(&root_dom);

    let needle = r#"
<ul class="targetList">
  <li class="key1"></li>
  <li class="key2"></li>
</ul>
"#;

    let res = dom.search(&needle).unwrap();

    assert_eq!(res.len(), 2);

    // ul: list1
    let ul_dom = res.get(0).unwrap();
    assert_eq!(DomType::Tag, ul_dom.dom_type);
    let tag = ul_dom.get_tag().unwrap();
    assert_eq!("ul".to_string(), tag.get_name());
    assert_eq!(Some("list1".to_string()), tag.get_attr("id"));
    assert_eq!(Some("targetList".to_string()), tag.get_attr("class"));

    // li key1
    let li_dom = ul_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Tag, li_dom.dom_type);
    let tag = li_dom.get_tag().unwrap();
    assert_eq!("li".to_string(), tag.get_name());
    assert_eq!(Some("key1".to_string()), tag.get_attr("class"));

    // text: 1-1
    let text_dom = li_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Text, text_dom.dom_type);
    let text = text_dom.get_text().unwrap();
    assert_eq!("1-1", text.get_text());

    // li key2
    let li_dom = ul_dom.get_children().unwrap().get(1).unwrap().clone();
    assert_eq!(DomType::Tag, li_dom.dom_type);
    let tag = li_dom.get_tag().unwrap();
    assert_eq!("li".to_string(), tag.get_name());
    assert_eq!(Some("key2".to_string()), tag.get_attr("class"));

    // span
    let span_dom = li_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Tag, span_dom.dom_type);
    let tag = span_dom.get_tag().unwrap();
    assert_eq!("span".to_string(), tag.get_name());

    // text: 1-2
    let text_dom = span_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Text, text_dom.dom_type);
    let text = text_dom.get_text().unwrap();
    assert_eq!("1-2", text.get_text());

    // ul: list3
    let ul_dom = res.get(1).unwrap();
    assert_eq!(DomType::Tag, ul_dom.dom_type);
    let tag = ul_dom.get_tag().unwrap();
    assert_eq!("ul".to_string(), tag.get_name());
    assert_eq!(Some("list3".to_string()), tag.get_attr("id"));
    assert_eq!(Some("targetList".to_string()), tag.get_attr("class"));

    // li key1
    let li_dom = ul_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Tag, li_dom.dom_type);
    let tag = li_dom.get_tag().unwrap();
    assert_eq!("li".to_string(), tag.get_name());
    assert_eq!(Some("key1".to_string()), tag.get_attr("class"));

    // text: 3-1
    let text_dom = li_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Text, text_dom.dom_type);
    let text = text_dom.get_text().unwrap();
    assert_eq!("3-1", text.get_text());

    // li item
    let li_dom = ul_dom.get_children().unwrap().get(1).unwrap().clone();
    assert_eq!(DomType::Tag, li_dom.dom_type);
    let tag = li_dom.get_tag().unwrap();
    assert_eq!("li".to_string(), tag.get_name());
    assert_eq!(Some("item".to_string()), tag.get_attr("class"));

    // text: 3-2
    let text_dom = li_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Text, text_dom.dom_type);
    let text = text_dom.get_text().unwrap();
    assert_eq!("3-2", text.get_text());

    // li key2
    let li_dom = ul_dom.get_children().unwrap().get(2).unwrap().clone();
    assert_eq!(DomType::Tag, li_dom.dom_type);
    let tag = li_dom.get_tag().unwrap();
    assert_eq!("li".to_string(), tag.get_name());
    assert_eq!(Some("key2".to_string()), tag.get_attr("class"));

    // text: 3-3
    let text_dom = li_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Text, text_dom.dom_type);
    let text = text_dom.get_text().unwrap();
    assert_eq!("3-3", text.get_text());
}
