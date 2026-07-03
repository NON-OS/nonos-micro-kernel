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

use super::abs_out_of_flow::out_of_flow;
use super::image_box::image_box;
use super::inline_items::InlineItem;
use super::tree::{BoxKind, BoxNode};

const MAX_DEPTH: u32 = 400;

// Flatten an inline subtree into measured words, images and hard breaks.
// A stray block inside an inline run flows like its children.
pub(super) fn collect_items(
    children: &[BoxNode],
    content_w: i32,
    out: &mut Vec<InlineItem>,
    depth: u32,
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
                let measure = |s: &str| {
                    if mono {
                        nonos_app_skeleton::measure_ttf_mono(s, fpx)
                    } else {
                        nonos_app_skeleton::measure_ttf(s, fpx)
                    }
                };
                let space = measure(" ").max(1);
                let line_h = c.style.line_height() as i32;
                for w in t.split_whitespace() {
                    let adv = measure(w).max(0) + if c.style.bold { 1 } else { 0 };
                    out.push(InlineItem::Word {
                        text: String::from(w),
                        px,
                        color: c.style.color,
                        bg: c.style.bg,
                        bold: c.style.bold,
                        mono,
                        href: c.href.clone(),
                        adv,
                        space,
                        line_h,
                        node: c.dom_id,
                    });
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
                });
            }
            BoxKind::Inline | BoxKind::Block | BoxKind::Flex | BoxKind::Grid => {
                collect_items(&c.children, content_w, out, depth + 1);
            }
        }
    }
}
