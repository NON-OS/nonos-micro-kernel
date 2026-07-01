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

const IMAGE_W: u32 = 220;
const IMAGE_H: u32 = 82;
const IMAGE_FG: u32 = 0xFF8C_DFEA;

pub(super) fn emit_image(lines: &mut Vec<RenderLine>, line: &mut RenderLine, cur: &mut Cursor, src: &str, alt: &str) {
    if cur.x + IMAGE_W > cur.width.saturating_sub(MARGIN) && !line.spans.is_empty() {
        cur.y += line.height + 6;
        lines.push(core::mem::replace(line, RenderLine { y: cur.y, height: LINE_H, spans: Vec::new() }));
        cur.x = MARGIN;
    }
    let label = if alt.is_empty() { "[image]" } else { alt };
    line.height = line.height.max(IMAGE_H);
    line.spans.push(Span {
        x: cur.x, w: IMAGE_W, text: label.to_string(), color: IMAGE_FG,
        href: None, image_src: Some(String::from(src)), scale: 1, bold: false,
    });
    cur.x = cur.x.saturating_add(IMAGE_W + MARGIN);
}
