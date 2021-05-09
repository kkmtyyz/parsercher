use parsercher;
use parsercher::dom::Tag;

fn main() {
    let mut p = Tag::new("h1".to_string());
    p.set_attr("class", "target");

    let mut q = Tag::new("h1".to_string());
    q.set_attr("id", "q");
    q.set_attr("class", "target");

    assert_eq!(parsercher::satisfy_sufficient_condition(&p, &q), true);

    let mut q = Tag::new("h1".to_string());
    q.set_attr("id", "q");
    assert_eq!(parsercher::satisfy_sufficient_condition(&p, &q), false);
}
