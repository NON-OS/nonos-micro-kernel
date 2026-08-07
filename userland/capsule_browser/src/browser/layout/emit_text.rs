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

use super::doc::{RenderLine, LINE_H};
use super::wrap;

pub(super) fn emit_text(
    lines: &mut Vec<RenderLine>,
    line: &mut RenderLine,
    cur: &mut wrap::Cursor,
    text: &str,
    href: Option<String>,
    scale: u32,
    bold: bool,
    color: u32,
    bg: u32,
) {
    // Only ASCII spaces separate words. A non breaking space is whitespace
    // to Unicode but the one character an author uses to say "do not break
    // here", so splitting on it undoes exactly what it was written for.
    for w in text.split_ascii_whitespace() {
        let (span, wrapped) = wrap::word(cur, w, href.clone(), scale, bold, color, bg);
        if wrapped {
            lines.push(core::mem::replace(
                line,
                RenderLine { y: cur.y, height: LINE_H, spans: Vec::new() },
            ));
        }
        if super::line_height::line_height(scale) > line.height {
            line.height = super::line_height::line_height(scale);
        }
        line.spans.push(span);
    }
}
