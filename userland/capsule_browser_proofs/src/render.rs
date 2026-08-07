// NONOS Operating System (AGPL-3.0-or-later)
//! Drive the real render pipeline end to end: parse, cascade, box tree,
//! layout. Same call order the capsule uses, so a geometry proof here is a
//! statement about what the device paints.

use crate::browser::css::{collect_css, compute};
use crate::browser::dom;
use crate::browser::layout::boxmodel::{build, layout, BoxDocument, Content, Fragment};

pub fn render(html: &str, viewport_w: u32) -> BoxDocument {
    let d = dom::parse(html.as_bytes());
    let css = collect_css(&d);
    let s = compute(&d, &css);
    let root = build(&d, &s.styles, &s.bg_images, &s.grids, &s.pseudos);
    layout(&root, viewport_w)
}

// Text a fragment paints, for tests that care about where a word landed.
pub fn text_of(f: &Fragment) -> Option<&str> {
    match &f.content {
        Content::Text { text, .. } => Some(text.as_str()),
        _ => None,
    }
}

// Every text fragment as (x, y, w, text), in paint order.
pub fn texts(doc: &BoxDocument) -> alloc::vec::Vec<(i32, i32, i32, &str)> {
    doc.frags.iter().filter_map(|f| text_of(f).map(|t| (f.x, f.y, f.w, t))).collect()
}
