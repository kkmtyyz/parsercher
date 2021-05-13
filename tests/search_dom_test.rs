extern crate parsercher;

use parsercher::dom::Dom;
use parsercher::dom::DomType;
use parsercher::dom::Tag;

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

    // <ul class="targetList">
    let mut ul_dom = Dom::new(DomType::Tag);
    let mut ul_tag = Tag::new("ul".to_string());
    ul_tag.set_attr("class", "targetList");
    ul_dom.set_tag(ul_tag);

    // <li class="key1">
    let mut li_dom1 = Dom::new(DomType::Tag);
    let mut li_tag = Tag::new("li".to_string());
    li_tag.set_attr("class", "key1");
    li_dom1.set_tag(li_tag);

    // <li class="key2">
    let mut li_dom2 = Dom::new(DomType::Tag);
    let mut li_tag = Tag::new("li".to_string());
    li_tag.set_attr("class", "key2");
    li_dom2.set_tag(li_tag);

    // <ul class="targetList">
    //   <li class="key1"></li>
    //   <li class="key2"></li>
    // </ul>
    ul_dom.add_child(li_dom1);
    ul_dom.add_child(li_dom2);
    //parsercher::print_dom_tree(&ul_dom);

    let root_dom = parsercher::search_dom(&dom, &ul_dom).unwrap();

    // root
    assert_eq!(DomType::Tag, root_dom.dom_type);
    let tag = root_dom.get_tag().unwrap();
    assert_eq!("root".to_string(), tag.get_name());
    assert_eq!(None, tag.get_attrs());

    // ul
}
