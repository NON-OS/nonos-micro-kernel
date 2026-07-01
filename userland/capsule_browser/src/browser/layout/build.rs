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

use crate::browser::html::flow::Flow;

use super::doc::{RenderDocument, RenderLine, LINE_H};
use super::wrap::{Cursor, MARGIN};

const PARA_GAP: u32 = 6;

pub fn build(flows: &[Flow], width: u32, advance: u32) -> RenderDocument {
    let mut lines: Vec<RenderLine> = Vec::new();
    let mut cur = Cursor { x: MARGIN, y: MARGIN, width, advance };
    let mut line = RenderLine { y: cur.y, height: LINE_H, spans: Vec::new() };
    for f in flows {
        match f {
            Flow::Break => {
                if !line.spans.is_empty() {
                    cur.y += line.height + PARA_GAP;
                    lines.push(core::mem::replace(
                        &mut line,
                        RenderLine { y: cur.y, height: LINE_H, spans: Vec::new() },
                    ));
                    cur.x = MARGIN;
                }
            }
            Flow::Text(t, style) => {
                if style.pre {
                    super::emit_pre::emit_pre(&mut lines, &mut line, &mut cur, t);
                    continue;
                }
                let (scale, bold) = super::span_metrics::span_metrics(style);
                super::emit_text::emit_text(&mut lines, &mut line, &mut cur, t, None, scale, bold);
            }
            Flow::Link(t, href) => {
                let shown = if t.is_empty() { href.clone() } else { t.clone() };
                super::emit_text::emit_text(&mut lines, &mut line, &mut cur, &shown, Some(href.clone()), 1, false);
            }
            Flow::Image(src, alt) => {
                super::emit_image::emit_image(&mut lines, &mut line, &mut cur, src, alt);
            }
        }
    }
    lines.push(line);
    let content_h = cur.y + LINE_H;
    RenderDocument { lines, content_h }
}
