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

use crate::doc::document::Doc;
use crate::doc::export::pdf::fmt::{push_f32, push_usize};
use crate::doc::export::pdf::text::push_literal;
use crate::doc::page::{Page, PageMetrics};
use crate::doc::style::{Family, RunStyle};

pub fn font_index(style: &RunStyle) -> usize {
    let base = match style.family {
        Family::Sans => 1,
        Family::Mono => 3,
    };
    base + if style.bold { 1 } else { 0 }
}

pub fn baseline(pm: &PageMetrics, y: f32, ascent: f32) -> f32 {
    pm.height - pm.margin - y - ascent
}

pub fn page_content(doc: &Doc, page: &Page, pm: &PageMetrics) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"BT\n");
    for line in &page.lines {
        let block = match doc.blocks.get(line.block) {
            Some(b) => b,
            None => continue,
        };
        let style = block.style_at(0);
        let text = block.as_str().get(line.start..line.end).unwrap_or("");
        out.extend_from_slice(b"/F");
        push_usize(&mut out, font_index(&style));
        out.push(b' ');
        push_f32(&mut out, style.size_px);
        out.extend_from_slice(b" Tf\n");
        push_rgb(&mut out, style.color);
        out.extend_from_slice(b"1 0 0 1 ");
        push_f32(&mut out, pm.margin);
        out.push(b' ');
        push_f32(&mut out, baseline(pm, line.y, line.ascent));
        out.extend_from_slice(b" Tm\n");
        push_literal(&mut out, text);
        out.extend_from_slice(b" Tj\n");
    }
    out.extend_from_slice(b"ET\n");
    out
}

fn push_rgb(out: &mut Vec<u8>, color: u32) {
    for shift in [16u32, 8, 0] {
        push_f32(out, ((color >> shift) & 0xFF) as f32 / 255.0);
        out.push(b' ');
    }
    out.extend_from_slice(b"rg\n");
}
