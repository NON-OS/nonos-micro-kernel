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

mod escape;
mod inline;

use alloc::format;
use alloc::string::String;

use crate::doc::document::Doc;
use crate::doc::kind::BlockKind;

pub fn to_markdown(doc: &Doc) -> String {
    let mut out = String::new();
    let mut number = 0usize;
    for block in &doc.blocks {
        if block.kind == BlockKind::Numbered {
            number += 1;
        } else {
            number = 0;
        }
        if !out.is_empty() {
            out.push_str("\n\n");
        }
        push_prefix(block.kind, number, &mut out);
        inline::push_inline(block, &mut out);
    }
    if !out.is_empty() {
        out.push('\n');
    }
    out
}

fn push_prefix(kind: BlockKind, number: usize, out: &mut String) {
    match kind {
        BlockKind::Heading(level) => {
            for _ in 0..level.clamp(1, 6) {
                out.push('#');
            }
            out.push(' ');
        }
        BlockKind::Bullet => out.push_str("- "),
        BlockKind::Numbered => out.push_str(&format!("{}. ", number)),
        BlockKind::PageBreak => out.push_str("---"),
        BlockKind::Paragraph => {}
    }
}
