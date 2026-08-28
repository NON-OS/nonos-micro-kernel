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

use crate::doc::document::Doc;
use crate::doc::linebox::LineBox;
use crate::doc::measure::Measurer;
use crate::doc::page::Page;

pub fn line_for(page: &Page, block: usize, off: usize) -> Option<&LineBox> {
    page.lines
        .iter()
        .find(|l| l.block == block && off >= l.start && off <= l.end)
}

pub fn caret_rect(
    page: &Page,
    doc: &Doc,
    block: usize,
    off: usize,
    m: &dyn Measurer,
) -> Option<(f32, f32, f32)> {
    let line = line_for(page, block, off)?;
    let b = doc.blocks.get(block)?;
    let style = b.style_at(line.start);
    let x = m.advance(&b.as_str()[line.start..off], &style);
    Some((x, line.y, line.height))
}

pub fn caret_at(page: &Page, doc: &Doc, x: f32, y: f32, m: &dyn Measurer) -> (usize, usize) {
    let line = page
        .lines
        .iter()
        .find(|l| y >= l.y && y < l.y + l.height)
        .or_else(|| match page.lines.first() {
            Some(f) if y < f.y => Some(f),
            _ => page.lines.last(),
        });
    let line = match line {
        Some(l) => l,
        None => return (0, 0),
    };
    let b = match doc.blocks.get(line.block) {
        Some(b) => b,
        None => return (line.block, line.start),
    };
    let style = b.style_at(line.start);
    let text = &b.as_str()[line.start..line.end];
    let mut best = line.start;
    let mut best_d = f32::MAX;
    for (i, _) in text.char_indices().chain(core::iter::once((text.len(), ' '))) {
        let cx = m.advance(&text[..i], &style);
        let d = (cx - x).abs();
        if d < best_d {
            best_d = d;
            best = line.start + i;
        }
    }
    (line.block, best)
}
