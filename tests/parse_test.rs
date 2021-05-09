extern crate parsercher;

use parsercher::dom::DomType;

#[test]
fn parse() {
    let html = r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <title>sample html</title>
  </head>
  <body>
    <h1>Hello, world!</h1>

    <div id="content"></div>

    <ol>
      <li id="first" class="listItem">first</li>
      <li class="listItem">second</li>
      <li id="therd" class="listItem">therd</li>
    </ol>
    <!-- All script code becomes one text -->
<script>
  let content = document.getElementById('content');
  content.textContent = 'content';
</script>
  </body>
</html>
"#;

    let root_dom = parsercher::parse(&html).unwrap();

    // root
    assert_eq!(DomType::Tag, root_dom.dom_type);
    let tag = root_dom.get_tag().unwrap();
    assert_eq!("root".to_string(), tag.get_name());
    assert_eq!(None, tag.get_attrs());

    // doctype
    let doctype_dom = root_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Tag, doctype_dom.dom_type);
    let tag = doctype_dom.get_tag().unwrap();
    assert_eq!("!DOCTYPE".to_string(), tag.get_name());
    assert_eq!(Some("".to_string()), tag.get_attr("html"));

    // html
    let html_dom = root_dom.get_children().unwrap().get(1).unwrap().clone();
    assert_eq!(DomType::Tag, html_dom.dom_type);
    let tag = html_dom.get_tag().unwrap();
    assert_eq!("html".to_string(), tag.get_name());
    assert_eq!(None, tag.get_attrs());

    // head
    let head_dom = html_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Tag, head_dom.dom_type);
    let tag = head_dom.get_tag().unwrap();
    assert_eq!("head".to_string(), tag.get_name());
    assert_eq!(None, tag.get_attrs());

    // meta
    let meta_dom = head_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Tag, meta_dom.dom_type);
    let tag = meta_dom.get_tag().unwrap();
    assert_eq!("meta".to_string(), tag.get_name());
    assert_eq!(Some("UTF-8".to_string()), tag.get_attr("charset"));

    // title
    let title_dom = head_dom.get_children().unwrap().get(1).unwrap().clone();
    assert_eq!(DomType::Tag, title_dom.dom_type);
    let tag = title_dom.get_tag().unwrap();
    assert_eq!("title".to_string(), tag.get_name());
    assert_eq!(None, tag.get_attrs());

    // text "sample html"
    let text_dom = title_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Text, text_dom.dom_type);
    let text = text_dom.get_text().unwrap();
    assert_eq!("sample html", text.get_text());

    // body
    let body_dom = html_dom.get_children().unwrap().get(1).unwrap().clone();
    assert_eq!(DomType::Tag, body_dom.dom_type);
    let tag = body_dom.get_tag().unwrap();
    assert_eq!("body".to_string(), tag.get_name());
    assert_eq!(None, tag.get_attrs());

    // h1
    let h1_dom = body_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Tag, h1_dom.dom_type);
    let tag = h1_dom.get_tag().unwrap();
    assert_eq!("h1".to_string(), tag.get_name());
    assert_eq!(None, tag.get_attrs());

    // text "Hello, world!"
    let text_dom = h1_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Text, text_dom.dom_type);
    let text = text_dom.get_text().unwrap();
    assert_eq!("Hello, world!", text.get_text());

    // div
    let div_dom = body_dom.get_children().unwrap().get(1).unwrap().clone();
    assert_eq!(DomType::Tag, div_dom.dom_type);
    let tag = div_dom.get_tag().unwrap();
    assert_eq!("div".to_string(), tag.get_name());
    assert_eq!("content", tag.get_attr("id").unwrap());

    // ol
    let ol_dom = body_dom.get_children().unwrap().get(2).unwrap().clone();
    assert_eq!(DomType::Tag, ol_dom.dom_type);
    let tag = ol_dom.get_tag().unwrap();
    assert_eq!("ol".to_string(), tag.get_name());
    assert_eq!(None, tag.get_attrs());

    // li first
    let li_first_dom = ol_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Tag, li_first_dom.dom_type);
    let tag = li_first_dom.get_tag().unwrap();
    assert_eq!("li".to_string(), tag.get_name());
    assert_eq!("listItem".to_string(), tag.get_attr("class").unwrap());
    assert_eq!("first".to_string(), tag.get_attr("id").unwrap());

    // li second
    let li_second_dom = ol_dom.get_children().unwrap().get(1).unwrap().clone();
    assert_eq!(DomType::Tag, li_second_dom.dom_type);
    let tag = li_second_dom.get_tag().unwrap();
    assert_eq!("li".to_string(), tag.get_name());
    assert_eq!("listItem".to_string(), tag.get_attr("class").unwrap());

    // li therd
    let li_therd_dom = ol_dom.get_children().unwrap().get(2).unwrap().clone();
    assert_eq!(DomType::Tag, li_therd_dom.dom_type);
    let tag = li_therd_dom.get_tag().unwrap();
    assert_eq!("li".to_string(), tag.get_name());
    assert_eq!("listItem".to_string(), tag.get_attr("class").unwrap());
    assert_eq!("therd".to_string(), tag.get_attr("id").unwrap());

    // comment " All script code becomes one text "
    let comment_dom = body_dom.get_children().unwrap().get(3).unwrap().clone();
    assert_eq!(DomType::Comment, comment_dom.dom_type);
    let comment = comment_dom.get_comment().unwrap();
    assert_eq!(" All script code becomes one text ", comment.get_comment());

    // script
    let script_dom = body_dom.get_children().unwrap().get(4).unwrap().clone();
    assert_eq!(DomType::Tag, script_dom.dom_type);
    let tag = script_dom.get_tag().unwrap();
    assert_eq!("script".to_string(), tag.get_name());
    assert_eq!(None, tag.get_attrs());

    //  text "\n  let content = document.getElementById('content');\n  content.textContent = 'content';\n"
    let text_dom = script_dom.get_children().unwrap().get(0).unwrap().clone();
    assert_eq!(DomType::Text, text_dom.dom_type);
    let text = text_dom.get_text().unwrap();
    assert_eq!("\n  let content = document.getElementById('content');\n  content.textContent = 'content';\n", text.get_text());
}
