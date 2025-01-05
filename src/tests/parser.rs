use crate::parser::*;
use crate::types::*;
use pretty_assertions::assert_eq;

macro_rules! node {
    ($tag:ident) => {
        EmmetNode::from_tag(stringify!($tag))
    };

    ($parent:ident > $child:ident) => {
        EmmetNode::from_tag(stringify!($parent)).add_child(EmmetNode::from_tag(stringify!($child)))
    };

    ($parent:ident > $child:ident > $($rest:ident)>+) => {
        EmmetNode::from_tag(stringify!($parent))
            .add_child(node!($child > $($rest)>+))
    };

    ($first:ident + $second:ident) => {
        EmmetNode::from_tag(stringify!($first)).add_sibling(EmmetNode::from_tag(stringify!($second)))
    };

    ($first:ident + $second:ident + $($rest:ident)+) => {
        EmmetNode::from_tag(stringify!($first))
            .add_sibling(node!($second + $($rest)+))
    };

}

#[test]
fn simple_tag() {
    assert_eq!(parse("div").unwrap(), node! { div })
}

#[test]
fn nested_tags() {
    assert_eq!(parse("div>ul>li").unwrap(), node! {div > ul > li})
}

#[test]
fn sibling() {
    assert_eq!(parse("div+p").unwrap(), node! {div + p})
}

#[test]
fn siblings() {
    assert_eq!(parse("div+p+h1").unwrap(), node! {div + p + h1})
}

#[test]
fn nested_plus_siblings() {
    let p = node! {p}.add_child(node! {span + em});
    let sibling = node! {div}.add_child(p);

    let div = node! {div}.add_sibling(sibling);
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

#[test]
fn test_class() {
    let mut expected = EmmetNode::from_tag("div");
    expected.classes = vec!["container".to_string()];
    assert_eq!(parse("div.container").unwrap(), expected);
}

#[test]
fn test_multiple_classes() {
    let mut expected = EmmetNode::from_tag("div");
    expected.classes = vec![
        "container".to_string(),
        "wrapper".to_string(),
        "main".to_string(),
    ];
    assert_eq!(parse("div.container.wrapper.main").unwrap(), expected);
}

#[test]
fn test_id() {
    let mut expected = EmmetNode::from_tag("div");
    expected.id = Some("header".to_string());
    assert_eq!(parse("div#header").unwrap(), expected);
}

#[test]
fn test_class_and_id() {
    let mut expected = EmmetNode::from_tag("div");
    expected.id = Some("header".to_string());
    expected.classes = vec!["container".to_string()];
    assert_eq!(parse("div#header.container").unwrap(), expected);
}

#[test]
fn test_implicit_div() {
    let mut expected = EmmetNode::from_tag("div");
    expected.classes = vec!["container".to_string()];
    assert_eq!(parse(".container").unwrap(), expected);

    let mut expected = EmmetNode::from_tag("div");
    expected.id = Some("main".to_string());
    assert_eq!(parse("#main").unwrap(), expected);
}

#[test]
fn test_text_content() {
    let mut expected = EmmetNode::from_tag("p");
    expected.text = Some("Hello World".to_string());
    assert_eq!(parse("p{Hello World}").unwrap(), expected);
}

#[test]
fn test_complex_combination() {
    let expected = node! {div}
        .add()
        .id("main")
        .class("container")
        .text("Content")
        .call();

    assert_eq!(parse("div#main.container{Content}").unwrap(), expected);
}

#[test]
fn test_siblings_with_text() {
    let mut first = node! { div }.add().class("first").call();
    let second = node! { div }.add().class("second").text("Content").call();
    first.siblings.push(second);

    assert_eq!(parse("div.first+div.second{Content}").unwrap(), first);
}

#[test]
fn test_multiplication_with_classes() {
    let mut expected = node! {li}.add().class("item").call();
    expected.multiplier = 3;

    let mut parent = node! { ul };
    parent.children.push(expected);

    assert_eq!(parse("ul>li.item*3").unwrap(), parent);
}
