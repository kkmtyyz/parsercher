use parsercher;
use parsercher::dom::Tag;
use std::collections::HashMap;

fn main() {
    let mut p = Tag::new("h1".to_string());
    let mut attr = HashMap::new();
    attr.insert("class".to_string(), "target".to_string());
    p.set_attr(attr);

    let mut q = Tag::new("h1".to_string());
    let mut attr = HashMap::new();
    attr.insert("id".to_string(), "q".to_string());
    attr.insert("class".to_string(), "target".to_string());
    q.set_attr(attr);

    assert_eq!(parsercher::satisfy_sufficient_condition(&p, &q), true);

    let mut q = Tag::new("h1".to_string());
    let mut attr = HashMap::new();
    attr.insert("id".to_string(), "q".to_string());
    q.set_attr(attr);
    assert_eq!(parsercher::satisfy_sufficient_condition(&p, &q), false);
}
