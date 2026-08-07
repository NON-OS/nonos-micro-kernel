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

use alloc::vec::Vec;

use super::abs_out_of_flow::out_of_flow;
use super::border_box_w::border_box_w;
use super::content_width::content_width;
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::image_box::image_box;
use super::inline_items::InlineItem;
use super::layout_box::layout_box;
use crate::browser::css::{Size, WhiteSpace};

use super::tree::{BoxKind, BoxNode};

const MAX_DEPTH: u32 = 400;

// Flatten an inline subtree into measured words, images, inline-block atoms
// and hard breaks. A stray block inside an inline run flows like its children.
pub(super) fn collect_items(
    children: &[BoxNode],
    content_w: i32,
    out: &mut Vec<InlineItem>,
    depth: u32,
    ctx: Ctx,
) {
    if depth > MAX_DEPTH {
        return;
    }
    for c in children {
        if out_of_flow(&c.style) {
            continue;
        }
        match &c.kind {
            BoxKind::Text(t) => {
                if t == "\n" {
                    out.push(InlineItem::Break);
                    continue;
                }
                let px = c.style.font_size_px;
                let fpx = px as f32;
                let mono = c.style.mono;
                let font = c.style.font_key;
                let spacing = c.style.letter_spacing;
                let bold = c.style.bold;
                let measure = |s: &str| {
                    crate::browser::fonts::measure_text(font, mono, bold, s, fpx, spacing)
                };
                let space = measure(" ").max(1);
                let line_h = c.style.line_height() as i32;
                let tt = c.style.text_transform;
                let icon = c.style.icon_font;
                let word = |w: &str| {
                    // Icon-font text is a ligature name or private-use glyph we
                    // cannot draw: map it to a symbol the built-in face has, or
                    // emit nothing, so the icon never shows as a literal word.
                    let text = if icon {
                        alloc::string::String::from(
                            crate::browser::css::icon_font::map_ligature(w).unwrap_or(""),
                        )
                    } else {
                        super::text_transform::transform(w, tt)
                    };
                    InlineItem::Word {
                        px,
                        color: c.style.color,
                        bg: c.style.bg,
                        bold,
                        mono,
                        underline: c.style.underline,
                        font,
                        spacing,
                        href: c.href.clone(),
                        adv: measure(&text).max(0),
                        space,
                        line_h,
                        node: c.dom_id,
                        text,
                    }
                };
                if c.style.white_space == WhiteSpace::Pre {
                    // Preserve each line verbatim, breaking only at newlines, so
                    // code and pre-formatted text keep their spacing.
                    let mut first = true;
                    for line in t.split('\n') {
                        if !first {
                            out.push(InlineItem::Break);
                        }
                        first = false;
                        if !line.is_empty() {
                            out.push(word(line));
                        }
                    }
                } else {
                    for w in t.split_ascii_whitespace() {
                        out.push(word(w));
                    }
                }
            }
            BoxKind::Image { src, alt } => {
                let (w, h) = image_box(&c.style, content_w);
                out.push(InlineItem::Image {
                    src: src.clone(),
                    alt: alt.clone(),
                    w,
                    h,
                    href: c.href.clone(),
                    node: c.dom_id,
                    fit: c.style.object_fit,
                });
            }
            BoxKind::InlineBlock => {
                // An inline-block is an atom on the line. Give it its own
                // width (explicit, else shrink to content) and lay its block
                // context out at the origin; the line box shifts it into place.
                let bw = match c.style.width {
                    Size::Auto => content_width(c, depth + 1).clamp(1, content_w.max(1)),
                    _ => border_box_w(&c.style, content_w).clamp(1, content_w.max(1)),
                };
                let mut sub: DisplayList = Vec::new();
                let h = layout_box(c, 0, 0, bw, &mut sub, depth + 1, ctx);
                // The block may resolve wider than the hint (a min-width, an
                // unbreakable child), so the advance is the real laid-out
                // extent, or the next item would overlap it.
                let w = sub.iter().map(|f| f.x + f.w).max().unwrap_or(bw).max(bw);
                out.push(InlineItem::Atom { frags: sub, w, h });
            }
            BoxKind::Inline | BoxKind::Block | BoxKind::Flex | BoxKind::Grid => {
                collect_items(&c.children, content_w, out, depth + 1, ctx);
            }
        }
    }
}
