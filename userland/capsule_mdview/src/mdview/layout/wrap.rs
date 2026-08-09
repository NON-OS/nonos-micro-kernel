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

use super::block::{Block, Line, Span, Style};
use super::code::code_lines;
use super::metrics::{indent, px};
use super::word::push_word;

pub type Measure = fn(&str, f32, bool) -> i32;

pub fn wrap(blocks: &[Block], content_width: i32, measure: Measure) -> Vec<Line> {
    let mut lines = Vec::new();
    for block in blocks {
        if block.style == Style::Code {
            code_lines(block, &mut lines);
        } else {
            flow(block, (content_width - indent(block.style)).max(1), measure, &mut lines);
        }
    }
    lines
}

fn flow(block: &Block, limit: i32, measure: Measure, out: &mut Vec<Line>) {
    let size = px(block.style);
    let mut spans: Vec<Span> = Vec::new();
    let mut used = 0;
    let mut lead = true;
    for span in &block.spans {
        for word in span.text.split_whitespace() {
            let advance = measure(word, size, span.mono);
            let gap = measure(" ", size, span.mono);
            if !spans.is_empty() && used + gap + advance > limit {
                out.push(emit(block.style, core::mem::take(&mut spans), lead));
                lead = false;
                used = advance;
                push_word(&mut spans, word, span.mono, false);
                continue;
            }
            let separate = !spans.is_empty();
            used += if separate { gap + advance } else { advance };
            push_word(&mut spans, word, span.mono, separate);
        }
    }
    if !spans.is_empty() {
        out.push(emit(block.style, spans, lead));
    }
}

fn emit(style: Style, spans: Vec<Span>, lead: bool) -> Line {
    Line { style, spans, lead }
}
