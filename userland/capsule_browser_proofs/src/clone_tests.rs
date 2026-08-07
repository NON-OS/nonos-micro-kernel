// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs over copying a node and over the properties that are really
//! attributes: a template can become a row, and a link a script builds has a
//! destination.

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::parse;
use crate::browser::js::interp::attr_prop::{attr_prop, bool_prop};

#[test]
fn a_shallow_copy_leaves_the_contents_behind() {
    let mut dom = parse(b"<li class=\"row\"><span>text</span></li>");
    let row = dom.nodes.iter().position(|n| n.tag == "li").expect("row");
    let copy = dom.clone_node(row, false).expect("copy");

    assert_eq!(dom.nodes[copy].tag, "li");
    assert_eq!(dom.nodes[copy].attr("class"), Some("row"), "attributes were not copied");
    assert!(dom.nodes[copy].children.is_empty(), "a shallow copy took the subtree");
}

#[test]
fn a_deep_copy_takes_the_whole_subtree() {
    // The template case: a page writes the shape once and every row is a
    // copy of it.
    let mut dom = parse(b"<template><li class=\"row\"><span>name</span><b>x</b></li></template>");
    let row = dom.nodes.iter().position(|n| n.tag == "li").expect("row");
    let copy = dom.clone_node(row, true).expect("copy");

    let kids: Vec<String> =
        dom.nodes[copy].children.iter().map(|&c| dom.nodes[c].tag.clone()).collect();
    assert_eq!(kids, ["span", "b"], "the subtree did not come with it");
    let span = dom.nodes[copy].children[0];
    let text = dom.nodes[span].children.first().copied().expect("text");
    assert_eq!(dom.nodes[text].text, "name", "text was not copied");
}

#[test]
fn a_copy_is_not_in_the_document_until_it_is_placed() {
    // A copy the caller has not placed must not show up on the page. If it
    // did, every template a script copied would render twice.
    let mut dom = parse(b"<div><li>row</li></div>");
    let root = dom.nodes.iter().position(|n| n.tag == "div").expect("root");
    let row = dom.nodes.iter().position(|n| n.tag == "li").expect("row");
    let before = dom.nodes[root].children.len();

    let copy = dom.clone_node(row, true).expect("copy");
    assert_eq!(dom.nodes[root].children.len(), before, "the copy attached itself");

    assert!(dom.place(root, copy, usize::MAX));
    assert_eq!(dom.nodes[root].children.len(), before + 1);
}

#[test]
fn copying_a_copy_does_not_share_it() {
    // Writing to one copy must not change the other, or a list of rows built
    // from one template would show the same text in every row.
    let mut dom = parse(b"<li><span>a</span></li>");
    let row = dom.nodes.iter().position(|n| n.tag == "li").expect("row");
    let first = dom.clone_node(row, true).expect("first");
    let second = dom.clone_node(row, true).expect("second");
    assert_ne!(first, second);

    let inner = dom.nodes[first].children[0];
    let text = dom.nodes[inner].children[0];
    dom.nodes[text].text = "changed".into();

    let other_inner = dom.nodes[second].children[0];
    let other_text = dom.nodes[other_inner].children[0];
    assert_eq!(dom.nodes[other_text].text, "a", "the copies share a node");
}

#[test]
fn a_copy_of_a_text_node_carries_its_text() {
    let mut dom = parse(b"<p>hello</p>");
    let text = dom.nodes.iter().position(|n| n.kind == NodeKind::Text).expect("text");
    let copy = dom.clone_node(text, false).expect("copy");
    assert_eq!(dom.nodes[copy].text, "hello");
    assert!(dom.nodes[copy].kind == NodeKind::Text, "the kind was not copied");
}

#[test]
fn properties_name_the_attribute_markup_spells() {
    // The ones a script cannot guess, because the property and the attribute
    // are spelled differently.
    assert_eq!(attr_prop("htmlFor"), Some("for"));
    assert_eq!(attr_prop("tabIndex"), Some("tabindex"));
    assert_eq!(attr_prop("colSpan"), Some("colspan"));
    assert_eq!(attr_prop("maxLength"), Some("maxlength"));
    // And the ones that are the same, which matter most: a link with no
    // href and an image with no src are the whole point.
    assert_eq!(attr_prop("href"), Some("href"));
    assert_eq!(attr_prop("src"), Some("src"));
}

#[test]
fn presence_properties_are_not_string_properties() {
    // These have to be removed rather than set to "false". A property that
    // took the string route would leave a control disabled by the code that
    // meant to enable it.
    for name in ["disabled", "checked", "readOnly", "required", "hidden"] {
        assert!(bool_prop(name).is_some(), "{name} is not a presence property");
        assert!(attr_prop(name).is_none(), "{name} is in both tables");
    }
    assert!(bool_prop("href").is_none());
}
