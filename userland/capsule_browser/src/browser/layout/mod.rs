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

use crate::browser::html::flow::{Flow, Style};
use doc::{RenderDocument, RenderLine, LINE_H};
use wrap::{Cursor, MARGIN};

fn heading_scale(h: u8) -> u32 {
    match h {
        1 => 3,
        2 | 3 => 2,
        _ => 1,
    }
}

fn span_metrics(style: &Style) -> (u32, bool) {
    let scale = heading_scale(style.heading);
    let bold = style.bold || (style.heading >= 4 && style.heading <= 6);
    (scale, bold)
}

fn line_height(scale: u32) -> u32 {
    scale * 8 + 12
}

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
                let (scale, bold) = span_metrics(style);
                emit(&mut lines, &mut line, &mut cur, t, None, scale, bold);
            }
            Flow::Link(t, href) => {
                let shown = if t.is_empty() { href.clone() } else { t.clone() };
                emit(&mut lines, &mut line, &mut cur, &shown, Some(href.clone()), 1, false);
            }
            Flow::Image(_, alt) => emit(&mut lines, &mut line, &mut cur, alt, None, 1, false),
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
    scale: u32,
    bold: bool,
) {
    for w in text.split_whitespace() {
        let (span, wrapped) = wrap::word(cur, w, href.clone(), scale, bold);
        if wrapped {
            let finished = core::mem::replace(line, RenderLine { y: cur.y, height: LINE_H, spans: Vec::new() });
            lines.push(finished);
        }
        if line_height(scale) > line.height {
            line.height = line_height(scale);
        }
        line.spans.push(span);
    }
}
