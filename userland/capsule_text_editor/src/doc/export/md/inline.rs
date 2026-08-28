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
use super::escape::push_escaped;
use crate::doc::style::RunStyle;

pub fn push_inline(block: &Block, out: &mut String) {
    let text = block.as_str();
    let heading = matches!(block.kind, BlockKind::Heading(_));
    let mut at = 0usize;
    for run in &block.runs {
        let end = (at + run.len).min(text.len());
        let slice = if end > at { text.get(at..end) } else { None };
        at = end;
        let slice = match slice {
            Some(s) => s,
            None => continue,
        };
        push_open(&run.style, heading, out);
        push_escaped(slice, out);
        push_close(&run.style, heading, out);
    }
    if let Some(rest) = text.get(at..) {
        push_escaped(rest, out);
    }
}

fn push_open(style: &RunStyle, heading: bool, out: &mut String) {
    if style.strike {
        out.push_str("~~");
    }
    if style.bold && !heading {
        out.push_str("**");
    }
    if style.italic {
        out.push('*');
    }
}

fn push_close(style: &RunStyle, heading: bool, out: &mut String) {
    if style.italic {
        out.push('*');
    }
    if style.bold && !heading {
        out.push_str("**");
    }
    if style.strike {
        out.push_str("~~");
    }
}
