use parsercher;
use parsercher::dom::Tag;

fn main() {
    let mut p = Tag::new("h1");
    p.set_attr("class", "target");

    let mut q = Tag::new("h1");
    q.set_attr("id", "q");
    q.set_attr("class", "target");

    assert_eq!(Tag::p_implies_q(&p, &q), true);

    let mut q = Tag::new("h1");
    q.set_attr("id", "q");
    assert_eq!(Tag::p_implies_q(&p, &q), false);
}
