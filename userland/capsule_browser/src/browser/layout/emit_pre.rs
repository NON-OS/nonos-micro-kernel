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

use super::doc::{RenderLine, Span, LINE_H};
use super::wrap::{Cursor, MARGIN};

const FG: u32 = 0xFFE6_EDF3;

pub fn emit_pre(lines: &mut Vec<RenderLine>, line: &mut RenderLine, cur: &mut Cursor, text: &str) {
    let max_px = cur.width.saturating_sub(MARGIN * 2).max(cur.advance);
    let max_chars = (max_px / cur.advance).max(1) as usize;
    for chunk in text.as_bytes().chunks(max_chars) {
        let shown = String::from_utf8_lossy(chunk).into_owned();
        let w = shown.len() as u32 * cur.advance;
        line.spans.push(Span {
            x: MARGIN,
            w,
            text: shown,
            color: FG,
            href: None,
            image_src: None,
            scale: 1,
            bold: false,
        });
        lines.push(core::mem::replace(line, RenderLine { y: cur.y + LINE_H, height: LINE_H, spans: Vec::new() }));
        cur.y += LINE_H;
        cur.x = MARGIN;
    }
}
