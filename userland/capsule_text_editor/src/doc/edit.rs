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

use crate::doc::block::{Block, Run};
use crate::doc::style::RunStyle;

pub fn insert(block: &mut Block, off: usize, s: &str) {
    let off = off.min(block.text.len());
    let bytes = s.as_bytes();

    block.text.splice(off..off, bytes.iter().copied());

    let mut at = 0usize;
    for r in &mut block.runs {
        at += r.len;
        if off <= at {
            r.len += bytes.len();
            break;
        }
    }
}

pub fn delete(block: &mut Block, off: usize, n: usize) {
    let off = off.min(block.text.len());
    let n = n.min(block.text.len() - off);

    if n == 0 {
        return;
    }

    let end = off + n;
    block.text.drain(off..end);

    let mut at = 0usize;
    block.runs.retain_mut(|r| {
        let r_end = at + r.len;
        if r_end <= off {
            at = r_end;
            return true;
        }
        if at >= end {
            return true;
        }
        let trim_start = off.saturating_sub(at);
        let trim_end = end.min(r_end) - at;
        r.len -= trim_end - trim_start;
        at = end.min(r_end);
        r.len > 0
    });

    if block.runs.is_empty() {
        block.runs.push(Run { len: 0, style: RunStyle::body() });
    }
}
