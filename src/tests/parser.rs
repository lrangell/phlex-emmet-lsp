use crate::parser::*;
use crate::types::*;
use pretty_assertions::assert_eq;

#[test]
fn simple_tag() {
    assert_eq!(parse("div").unwrap(), EmmetNode::from_tag("div"))
}

#[test]
fn nested_tags() {
    let div = from_tag("div").add_child(from_tag("ul").add_child_str("li"));
    assert_eq!(parse("div>ul>li").unwrap(), div)
}

#[test]
fn siblings() {
    let div = from_tag("div").add_sibling(from_tag("p").add_sibling_str("h1"));
    assert_eq!(parse("div+p+h1").unwrap(), div)
}

#[test]
fn nested_plus_siblings() {
    let span_plus_em = from_tag("span").add_sibling_str("em");
    let p = from_tag("p").add_child(span_plus_em);
    let div2 = from_tag("div").add_child(p);

    let div = from_tag("div").add_sibling(div2);
    assert_eq!(parse("div+div>p>span+em").unwrap(), div);
}

#[test]
fn multiplication() {
    assert_eq!(
        parse("ul>li*3")
            .unwrap()
            .children
            .first()
            .unwrap()
            .multiplier,
        3
    );
}
