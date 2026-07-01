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

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::doc::{RenderLine, Span, LINE_H};
use super::wrap::{Cursor, MARGIN};

const IMAGE_H: u32 = 126;
const IMAGE_FG: u32 = 0xFF8C_DFEA;

pub(super) fn emit_image(lines: &mut Vec<RenderLine>, line: &mut RenderLine, cur: &mut Cursor, src: &str, alt: &str) {
    if !line.spans.is_empty() {
        cur.y += line.height + 6;
        lines.push(core::mem::replace(line, RenderLine { y: cur.y, height: LINE_H, spans: Vec::new() }));
        cur.x = MARGIN;
    }
    let label = if alt.is_empty() { "[image]" } else { alt };
    let image_w = cur.width.saturating_sub(MARGIN * 2).max(220);
    line.height = IMAGE_H;
    line.spans.push(Span {
        x: MARGIN, w: image_w, text: label.to_string(), color: IMAGE_FG,
        bg: 0, href: Some(String::from(src)), image_src: Some(String::from(src)), scale: 1, bold: false,
    });
    cur.y += IMAGE_H + 10;
    lines.push(core::mem::replace(line, RenderLine { y: cur.y, height: LINE_H, spans: Vec::new() }));
    cur.x = MARGIN;
}
