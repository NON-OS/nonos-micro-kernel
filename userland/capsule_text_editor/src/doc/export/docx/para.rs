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

use crate::doc::block::Block;
use crate::doc::kind::BlockKind;

use super::run::{escape, rpr};

pub fn pstyle(kind: BlockKind) -> Option<&'static str> {
    match kind {
        BlockKind::Paragraph | BlockKind::PageBreak => None,
        BlockKind::Heading(n) => Some(match n.clamp(1, 6) {
            1 => "Heading1",
            2 => "Heading2",
            3 => "Heading3",
            4 => "Heading4",
            5 => "Heading5",
            _ => "Heading6",
        }),
        BlockKind::Bullet => Some("ListBullet"),
        BlockKind::Numbered => Some("ListNumber"),
    }
}

pub fn paragraph(b: &Block, out: &mut String) {
    out.push_str("<w:p>");
    if let Some(style) = pstyle(b.kind) {
        out.push_str("<w:pPr><w:pStyle w:val=\"");
        out.push_str(style);
        out.push_str("\"/></w:pPr>");
    }
    let mut at = 0usize;
    for r in &b.runs {
        let end = (at + r.len).min(b.text.len());
        if end > at {
            let s = core::str::from_utf8(&b.text[at..end]).unwrap_or("");
            out.push_str("<w:r>");
            rpr(&r.style, out);
            out.push_str("<w:t xml:space=\"preserve\">");
            escape(s, out);
            out.push_str("</w:t></w:r>");
        }
        at = end;
    }
    out.push_str("</w:p>");
}
