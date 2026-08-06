// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs over the tree builder the capsule ships: a comment ends where the
//! specification says rather than at the first angle bracket inside it, raw
//! text elements keep their contents out of the tree, void elements do not
//! swallow what follows them, omitted end tags still nest, and a node can be
//! placed at a position rather than only at the end.

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::parse;

// Every bit of text in the document, in order, so a proof can say what a
// reader would actually see.
fn text_of(html: &str) -> String {
    let dom = parse(html.as_bytes());
    let mut out = String::new();
    for node in dom.nodes.iter() {
        if node.kind == NodeKind::Text {
            out.push_str(&node.text);
        }
    }
    out
}

// Tag names in tree order, so a proof can say what was built.
fn tags_of(html: &str) -> Vec<String> {
    let dom = parse(html.as_bytes());
    dom.nodes.iter().filter(|n| n.kind == NodeKind::Element).map(|n| n.tag.clone()).collect()
}

#[test]
fn a_comment_holding_markup_is_not_document_content() {
    // The case that matters: an author disabled a block, and the block
    // contains the character that ends a tag. Reading to the first one
    // leaves the rest of the comment as content.
    let html = "<p>before</p><!-- <div class=\"ad\">banner</div> --><p>after</p>";
    let text = text_of(html);
    assert!(!text.contains("banner"), "disabled markup was rendered: {text}");
    assert!(text.contains("before") && text.contains("after"));
    let tags = tags_of(html);
    assert!(!tags.iter().any(|t| t == "div"), "a commented out element was built: {tags:?}");
}

#[test]
fn a_comment_does_not_close_elements_around_it() {
    let html = "<div><!-- </div> --><span>inside</span></div>";
    let tags = tags_of(html);
    assert!(tags.iter().any(|t| t == "span"));
    // The close tag inside the comment must not have ended the div, which
    // would leave the span a sibling rather than a child.
    let dom = parse(html.as_bytes());
    let span = dom.nodes.iter().position(|n| n.tag == "span").expect("span");
    let parent = dom.nodes[span].parent;
    assert_eq!(dom.nodes[parent].tag, "div", "the comment closed the element around it");
}

#[test]
fn a_comment_ending_in_extra_dashes_still_closes() {
    let text = text_of("<p>a</p><!-- gone ---><p>b</p>");
    assert!(!text.contains("gone"));
    assert!(text.contains('a') && text.contains('b'));
}

#[test]
fn an_unterminated_comment_swallows_the_rest() {
    let text = text_of("<p>shown</p><!-- everything after this is a comment");
    assert!(text.contains("shown"));
    assert!(!text.contains("everything"));
}

#[test]
fn a_doctype_is_not_an_element() {
    let tags = tags_of("<!DOCTYPE html><html><body><p>hi</p></body></html>");
    assert!(!tags.iter().any(|t| t.starts_with('!')), "doctype became an element: {tags:?}");
    assert!(tags.iter().any(|t| t == "p"));
}

#[test]
fn script_contents_never_reach_the_tree() {
    // Script bodies contain characters that look like markup. They are the
    // program, not the document.
    let html = "<script>if (a < b && c > d) { x = \"</b>\"; }</script><p>after</p>";
    let tags = tags_of(html);
    assert!(!tags.iter().any(|t| t == "b"), "script text was parsed as markup: {tags:?}");
    assert!(tags.iter().any(|t| t == "p"));
}

#[test]
fn a_void_element_does_not_take_a_parent_role() {
    let html = "<div><img src=\"a.png\"><span>after</span></div>";
    let dom = parse(html.as_bytes());
    let span = dom.nodes.iter().position(|n| n.tag == "span").expect("span");
    let parent = dom.nodes[span].parent;
    assert_eq!(dom.nodes[parent].tag, "div", "the image adopted what followed it");
}

#[test]
fn omitted_end_tags_still_nest() {
    // Real HTML leaves these out constantly.
    let dom = parse(b"<ul><li>one<li>two<li>three</ul>");
    let items: Vec<usize> =
        dom.nodes.iter().enumerate().filter(|(_, n)| n.tag == "li").map(|(i, _)| i).collect();
    assert_eq!(items.len(), 3, "list items did not close each other");
    let first_parent = dom.nodes[items[0]].parent;
    for item in &items {
        assert_eq!(dom.nodes[*item].parent, first_parent, "items nested inside each other");
    }
}

#[test]
fn a_node_can_be_placed_rather_than_only_appended() {
    // What a reconciler does when a list reorders. Appending alone cannot
    // express it, so an update either lands in the wrong order or not at all.
    let mut dom = parse(b"<div id=\"root\"></div>");
    let root = dom.nodes.iter().position(|n| n.tag == "div").expect("root");

    let a = dom.create(NodeKind::Element, "a".into()).expect("a");
    let b = dom.create(NodeKind::Element, "b".into()).expect("b");
    let c = dom.create(NodeKind::Element, "c".into()).expect("c");
    assert!(dom.attach(root, a));
    assert!(dom.attach(root, b));

    // c ahead of b gives a, c, b.
    assert!(dom.insert_before(root, c, b));
    let order: Vec<String> =
        dom.nodes[root].children.iter().map(|&i| dom.nodes[i].tag.clone()).collect();
    assert_eq!(order, ["a", "c", "b"], "insertion ignored the position");
}

#[test]
fn placing_a_node_already_present_moves_it() {
    let mut dom = parse(b"<div></div>");
    let root = dom.nodes.iter().position(|n| n.tag == "div").expect("root");
    let a = dom.create(NodeKind::Element, "a".into()).expect("a");
    let b = dom.create(NodeKind::Element, "b".into()).expect("b");
    dom.attach(root, a);
    dom.attach(root, b);

    // Moving the first one to the end, the other direction from the case
    // above, so the index does not shift the same way.
    assert!(dom.insert_before(root, a, usize::MAX));
    let order: Vec<String> =
        dom.nodes[root].children.iter().map(|&i| dom.nodes[i].tag.clone()).collect();
    assert_eq!(order, ["b", "a"]);
    assert_eq!(dom.nodes[root].children.len(), 2, "the node was duplicated rather than moved");
}

#[test]
fn a_fragment_places_its_children_and_not_itself() {
    // A fragment exists so several nodes go in with one call. Placing the
    // holder would put a node in the document no markup ever described.
    let mut dom = parse(b"<div></div>");
    let root = dom.nodes.iter().position(|n| n.tag == "div").expect("root");
    let frag = dom.create(NodeKind::Document, String::new()).expect("fragment");
    let a = dom.create(NodeKind::Element, "a".into()).expect("a");
    let b = dom.create(NodeKind::Element, "b".into()).expect("b");
    dom.attach(frag, a);
    dom.attach(frag, b);

    assert!(dom.place(root, frag, usize::MAX));
    let order: Vec<String> =
        dom.nodes[root].children.iter().map(|&i| dom.nodes[i].tag.clone()).collect();
    assert_eq!(order, ["a", "b"], "the fragment did not unwrap");
    assert!(dom.nodes[frag].children.is_empty(), "the holder kept its children");
}

#[test]
fn a_reconciler_can_build_reorder_and_update_a_list() {
    // The whole cycle a framework drives: build a keyed list, reorder it,
    // replace an entry, and read back what is there. Each step needs a
    // different primitive, and any one missing shows up as an update that
    // silently does nothing.
    let mut dom = parse(b"<div id=\"app\"></div>");
    let app = dom.nodes.iter().position(|n| n.tag == "div").expect("app");

    // Render: three rows, each an element wrapping a text node.
    let mut rows = Vec::new();
    for label in ["one", "two", "three"] {
        let row = dom.create(NodeKind::Element, "li".into()).expect("row");
        let text = dom.create(NodeKind::Text, String::new()).expect("text");
        dom.nodes[text].text = label.into();
        assert!(dom.place(row, text, usize::MAX));
        assert!(dom.place(app, row, usize::MAX));
        rows.push(row);
    }
    let read = |dom: &crate::browser::dom::Dom| -> Vec<String> {
        dom.nodes[app]
            .children
            .iter()
            .map(|&r| {
                dom.nodes[r]
                    .children
                    .first()
                    .map(|&t| dom.nodes[t].text.clone())
                    .unwrap_or_default()
            })
            .collect()
    };
    assert_eq!(read(&dom), ["one", "two", "three"]);

    // Reorder: move the last row to the front, as a keyed diff does.
    assert!(dom.insert_before(app, rows[2], rows[0]));
    assert_eq!(read(&dom), ["three", "one", "two"]);

    // Update in place: the framework kept the text node and writes to it.
    let middle = dom.nodes[app].children[1];
    let text = dom.nodes[middle].children[0];
    dom.nodes[text].text = "ONE".into();
    assert_eq!(read(&dom), ["three", "ONE", "two"]);

    // Remove: drop the first row.
    let first = dom.nodes[app].children[0];
    dom.detach(first);
    assert_eq!(read(&dom), ["ONE", "two"]);
}
