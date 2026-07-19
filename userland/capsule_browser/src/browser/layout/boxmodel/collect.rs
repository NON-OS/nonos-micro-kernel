// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::Computed;
use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

use super::element::element;
use super::leaf::leaf;
use super::tree::{BoxKind, BoxNode};
use super::walk::{ElementIn, Walk};

pub(super) const MAX_DEPTH: u32 = 400;
pub(super) const MAX_BOXES: usize = 20_000;

// Build the box children of node `id`, recursing into elements. Text is kept
// when non-blank; elements go through the per-tag dispatcher. The li ordinal
// counts list items in this container for ordered-list markers.
#[allow(clippy::too_many_arguments)]
pub(super) fn collect(
    dom: &Dom,
    id: usize,
    parent: &Computed,
    styles: &[Computed],
    bg_images: &[Option<String>],
    grids: &[Option<crate::browser::css::GridSpec>],
    pseudos: &[(
        Option<crate::browser::css::PseudoText>,
        Option<crate::browser::css::PseudoText>,
    )],
    link: &Option<String>,
    depth: u32,
    count: &mut usize,
) -> Vec<BoxNode> {
    let mut out: Vec<BoxNode> = Vec::new();
    if depth > MAX_DEPTH {
        return out;
    }
    let Some(node) = dom.nodes.get(id) else {
        return out;
    };
    let mut w = Walk { dom, styles, bg_images, grids, pseudos, count };
    let mut ordinal = 0u32;
    // A closed <details> renders only its <summary>; the rest of the subtree
    // stays hidden until the open attribute appears.
    let closed_details = node.tag == "details" && node.attr("open").is_none();
    // A declarative shadow host renders its shadow tree, which layout skips.
    // Light children aimed at named slots (dropdown panels, overlays) would
    // paint over the page with no component to place them, so they skip too;
    // default-slot children (button labels) still render.
    let shadow_host = node.children.iter().any(|&t| {
        dom.nodes.get(t).is_some_and(|c| c.tag == "template" && c.attr("shadowrootmode").is_some())
    });
    for &ch in &node.children {
        if *w.count >= MAX_BOXES {
            break;
        }
        let Some(c) = dom.nodes.get(ch) else {
            continue;
        };
        if closed_details && !(c.kind == NodeKind::Element && c.tag == "summary") {
            continue;
        }
        match c.kind {
            NodeKind::Document => {}
            NodeKind::Text => {
                if !c.text.trim().is_empty() {
                    *w.count += 1;
                    out.push(leaf(BoxKind::Text(c.text.clone()), parent, link, ch));
                }
            }
            NodeKind::Element => {
                if shadow_host && c.attr("slot").is_some_and(|s| !s.is_empty()) {
                    continue;
                }
                // display: contents generates no box: the element's children
                // join this run directly (grid shells rely on this).
                let cs = w.styles.get(ch);
                if cs.is_some_and(|s| s.is_contents && !s.display_none) {
                    let inner = cs.copied().unwrap_or(*parent);
                    let spliced = collect(
                        dom,
                        ch,
                        &inner,
                        styles,
                        bg_images,
                        grids,
                        pseudos,
                        link,
                        depth + 1,
                        w.count,
                    );
                    out.extend(spliced);
                    continue;
                }
                if c.tag == "li" {
                    ordinal += 1;
                }
                let item = ElementIn { c, ch, parent_tag: &node.tag, ordinal };
                element(&mut w, &item, parent, link, depth, &mut out);
            }
        }
    }
    out
}
