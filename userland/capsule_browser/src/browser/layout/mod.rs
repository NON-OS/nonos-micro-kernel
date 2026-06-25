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

pub mod doc;
mod wrap;

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::html::flow::Flow;
use doc::{RenderDocument, RenderLine};
use wrap::{Cursor, LINE_H, MARGIN};

pub fn build(flows: &[Flow], width: u32, advance: u32) -> RenderDocument {
    let mut lines: Vec<RenderLine> = Vec::new();
    let mut cur = Cursor { x: MARGIN, y: MARGIN, width, advance };
    let mut line = RenderLine { y: cur.y, spans: Vec::new() };
    for f in flows {
        match f {
            Flow::Break => {
                lines.push(core::mem::replace(
                    &mut line,
                    RenderLine { y: cur.y + LINE_H, spans: Vec::new() },
                ));
                cur.y += LINE_H;
                cur.x = MARGIN;
            }
            Flow::Text(t, _) => emit(&mut lines, &mut line, &mut cur, t, None),
            Flow::Link(t, href) => {
                let shown = if t.is_empty() { href.clone() } else { t.clone() };
                emit(&mut lines, &mut line, &mut cur, &shown, Some(href.clone()));
            }
            Flow::Image(_, alt) => emit(&mut lines, &mut line, &mut cur, alt, None),
        }
    }
    lines.push(line);
    let content_h = cur.y + LINE_H;
    RenderDocument { lines, content_h }
}

fn emit(
    lines: &mut Vec<RenderLine>,
    line: &mut RenderLine,
    cur: &mut Cursor,
    text: &str,
    href: Option<String>,
) {
    for w in text.split_whitespace() {
        let (span, wrapped) = wrap::word(cur, w, href.clone());
        if wrapped {
            let finished = core::mem::replace(line, RenderLine { y: cur.y, spans: Vec::new() });
            lines.push(finished);
        }
        line.spans.push(span);
    }
}
