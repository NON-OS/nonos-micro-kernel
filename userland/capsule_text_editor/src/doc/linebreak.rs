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

use crate::doc::block::Block;
use crate::doc::linebox::LineBox;
use crate::doc::measure::Measurer;
use alloc::vec;
use alloc::vec::Vec;

pub fn break_block(block: &Block, idx: usize, max_w: f32, m: &dyn Measurer) -> Vec<LineBox> {
    let text = block.as_str();
    let style = block.style_at(0);
    let height = m.line_height(&style);
    let ascent = m.ascent(&style);
    if crate::doc::table::syntax::is_row(text) {
        let height = crate::doc::table::geom::row_height(&style, m);
        let b =
            LineBox { block: idx, start: 0, end: text.len(), width: max_w, height, ascent, y: 0.0 };
        return vec![b];
    }
    let mut lines = Vec::new();
    if text.is_empty() {
        return vec![LineBox { block: idx, start: 0, end: 0, width: 0.0, height, ascent, y: 0.0 }];
    }
    let mut pos = 0;
    while pos < text.len() {
        let start = pos;
        let mut best = start;
        let mut cut = next_break(text, start);
        loop {
            let cand = text[start..cut].trim_end();
            if m.advance(cand, &style) <= max_w {
                best = start + cand.len();
                if cut >= text.len() {
                    break;
                }
                cut = next_break(text, cut + 1);
            } else {
                break;
            }
        }
        if best == start {
            cut = next_break(text, start);
            best =
                start + text[start..if cut == start { text.len() } else { cut }].trim_end().len();
        }
        let t = text[start..best].trim_end();
        lines.push(LineBox {
            block: idx,
            start,
            end: best,
            width: m.advance(t, &style),
            height,
            ascent,
            y: 0.0,
        });
        pos = best;
        while pos < text.len() {
            match text[pos..].chars().next() {
                Some(c) if c.is_whitespace() => pos += c.len_utf8(),
                _ => break,
            }
        }
        if pos == start {
            pos = start + text[start..].chars().next().map_or(1, |c| c.len_utf8());
        }
    }
    lines
}

fn next_break(text: &str, pos: usize) -> usize {
    if pos >= text.len() {
        text.len()
    } else {
        text[pos..].find(' ').map(|p| pos + p).unwrap_or(text.len())
    }
}
